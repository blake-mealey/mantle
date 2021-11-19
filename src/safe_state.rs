use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use yansi::Paint;

use crate::{
    config::{
        AssetTargetConfig, Config, EnvironmentConfig, ExperienceTargetConfig, OwnerConfig,
        PlayabilityTargetConfig, RemoteStateConfig, StateConfig, TargetConfig,
    },
    logger,
    roblox_api::{RobloxApi, SocialLinkType},
    safe_resource_manager::*,
    safe_resources::ResourceGraph,
    state::{v1::ResourceStateV1, v2::ResourceStateV2, v3::ResourceStateV3},
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum ResourceState {
    Versioned(VersionedResourceState),
    Unversioned(ResourceStateV1),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "version")]
enum VersionedResourceState {
    #[serde(rename = "1")]
    V1(ResourceStateV1),
    #[serde(rename = "2")]
    V2(ResourceStateV2),
    #[serde(rename = "3")]
    V3(ResourceStateV3),
}

pub type ResourceStateVLatest = ResourceStateV3;

fn get_state_file_path(project_path: &Path) -> PathBuf {
    project_path.join(".mantle-state.yml")
}

fn get_hash(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    format!("{:x}", digest)
}

fn get_file_hash(file_path: PathBuf) -> Result<String, String> {
    let buffer = fs::read(&file_path).map_err(|e| {
        format!(
            "Failed to read file {} for hashing: {}",
            file_path.display(),
            e
        )
    })?;
    Ok(get_hash(&buffer))
}

fn parse_state(file_name: &str, data: &str) -> Result<ResourceState, String> {
    serde_yaml::from_str::<ResourceState>(data)
        .map_err(|e| format!("Unable to parse state file {}\n\t{}", file_name, e))
}

fn get_state_from_file(project_path: &Path) -> Result<Option<ResourceState>, String> {
    let state_file_path = get_state_file_path(project_path);
    logger::log(format!(
        "Loading previous state from local file {}",
        Paint::cyan(state_file_path.display())
    ));

    if state_file_path.exists() {
        let data = fs::read_to_string(&state_file_path).map_err(|e| {
            format!(
                "Unable to read state file: {}\n\t{}",
                state_file_path.display(),
                e
            )
        })?;

        return Ok(Some(parse_state(
            &state_file_path.display().to_string(),
            &data,
        )?));
    };

    Ok(None)
}

async fn get_state_from_remote(
    config: &RemoteStateConfig,
) -> Result<Option<ResourceState>, String> {
    logger::log(format!(
        "Loading previous state from remote object {}",
        Paint::cyan(config)
    ));

    let client = S3Client::new(config.region.clone());
    let object_res = client
        .get_object(rusoto_s3::GetObjectRequest {
            bucket: config.bucket.clone(),
            key: format!("{}.mantle-state.yml", config.key),
            ..Default::default()
        })
        .await;

    match object_res {
        Ok(object) => match object.body {
            Some(stream) => {
                let mut buffer = String::new();
                stream
                    .into_async_read()
                    .read_to_string(&mut buffer)
                    .await
                    .map_err(|_| "".to_owned())?;
                Ok(Some(parse_state(&format!("{}", config), &buffer)?))
            }
            _ => Ok(None),
        },
        Err(rusoto_core::RusotoError::Service(rusoto_s3::GetObjectError::NoSuchKey(_))) => Ok(None),
        Err(e) => Err(format!("Failed to get state from remote: {}", e)),
    }
}

pub async fn get_previous_state(
    project_path: &Path,
    config: &Config,
    environment_config: &EnvironmentConfig,
) -> Result<ResourceStateVLatest, String> {
    let state = match config.state.clone() {
        StateConfig::Local => get_state_from_file(project_path)?,
        StateConfig::Remote(config) => get_state_from_remote(&config).await?,
    };

    // Migrate previous state formats
    let mut state = match state {
        Some(ResourceState::Unversioned(state)) => {
            ResourceStateV3::from(ResourceStateV2::from(state))
        }
        Some(ResourceState::Versioned(VersionedResourceState::V1(state))) => {
            ResourceStateV3::from(ResourceStateV2::from(state))
        }
        Some(ResourceState::Versioned(VersionedResourceState::V2(state))) => {
            ResourceStateV3::from(state)
        }
        Some(ResourceState::Versioned(VersionedResourceState::V3(state))) => state,
        None => ResourceStateVLatest {
            environments: HashMap::new(),
        },
    };

    if state.environments.get(&environment_config.name).is_none() {
        logger::log(format!(
            "No previous state for environment {}",
            Paint::cyan(environment_config.name.clone())
        ));
        state
            .environments
            .insert(environment_config.name.clone(), Vec::new());
    }

    Ok(state)
}

fn get_desired_experience_graph(
    project_path: &Path,
    target_config: &ExperienceTargetConfig,
    owner_config: &OwnerConfig,
) -> Result<ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>, String> {
    let mut resources: Vec<RobloxResource> = Vec::new();

    let group_id = match owner_config {
        OwnerConfig::Personal => None,
        OwnerConfig::Group(group_id) => Some(*group_id),
    };

    let experience = RobloxResource::new(
        "experience_singleton",
        RobloxInputs::Experience(ExperienceInputs { group_id }),
        &[],
    );
    resources.push(experience.clone());

    resources.push(RobloxResource::new(
        "experienceActivation_singleton",
        RobloxInputs::ExperienceActivation(ExperienceActivationInputs {
            is_active: !matches!(
                target_config
                    .configuration
                    .as_ref()
                    .and_then(|c| c.playability)
                    .unwrap_or(PlayabilityTargetConfig::Private),
                PlayabilityTargetConfig::Private
            ),
        }),
        &[&experience],
    ));

    if let Some(experience_configuration) = &target_config.configuration {
        resources.push(RobloxResource::new(
            "experienceConfiguration_singleton",
            RobloxInputs::ExperienceConfiguration(experience_configuration.into()),
            &[&experience],
        ));

        if let Some(icon_path) = &experience_configuration.icon {
            resources.push(RobloxResource::new(
                "experienceIcon_singleton",
                RobloxInputs::ExperienceIcon(FileInputs {
                    file_path: icon_path.clone(),
                    file_hash: get_file_hash(project_path.join(icon_path))?,
                }),
                &[&experience],
            ));
        }

        if let Some(thumbnails) = &experience_configuration.thumbnails {
            let mut thumbnail_resources: Vec<RobloxResource> = Vec::new();
            for thumbnail_path in thumbnails {
                thumbnail_resources.push(RobloxResource::new(
                    &format!("experienceThumbnail_{}", thumbnail_path),
                    RobloxInputs::ExperienceThumbnail(FileInputs {
                        file_path: thumbnail_path.clone(),
                        file_hash: get_file_hash(project_path.join(thumbnail_path))?,
                    }),
                    &[&experience],
                ));
            }
            let mut thumbnail_order_dependencies: Vec<&RobloxResource> =
                thumbnail_resources.iter().collect();
            thumbnail_order_dependencies.push(&experience);
            resources.push(RobloxResource::new(
                "experienceThumbnailOrder_singleton",
                RobloxInputs::ExperienceThumbnailOrder,
                &thumbnail_order_dependencies,
            ));
            resources.extend(thumbnail_resources);
        }
    }

    if let Some(places) = &target_config.places {
        if !places.contains_key("start") {
            return Err("No start place specified".to_owned());
        }

        for (name, place) in places.iter() {
            let place_resource = RobloxResource::new(
                &format!("place_{}", name),
                RobloxInputs::Place(PlaceInputs {
                    is_start: name == "start",
                }),
                &[&experience],
            );
            resources.push(place_resource.clone());

            if let Some(file) = &place.file {
                resources.push(RobloxResource::new(
                    &format!("placeFile_{}", name),
                    RobloxInputs::PlaceFile(FileInputs {
                        file_path: file.clone(),
                        file_hash: get_file_hash(project_path.join(file))?,
                    }),
                    &[&place_resource],
                ));
            }

            if let Some(configuration) = &place.configuration {
                resources.push(RobloxResource::new(
                    &format!("placeConfiguration_{}", name),
                    RobloxInputs::PlaceConfiguration(configuration.clone().into()),
                    &[&place_resource],
                ));
            }
        }
    } else {
        return Err("No start place specified".to_owned());
    }

    if let Some(social_links) = &target_config.social_links {
        for social_link in social_links {
            let domain = social_link.url.domain().ok_or(format!(
                "Unknown social link type for URL {}",
                social_link.url
            ))?;
            let link_type = match domain {
                "facebook.com" => SocialLinkType::Facebook,
                "twitter.com" => SocialLinkType::Twitter,
                "youtube.com" => SocialLinkType::YouTube,
                "twitch.tv" => SocialLinkType::Twitch,
                "discord.gg" => SocialLinkType::Discord,
                "roblox.com" => SocialLinkType::RobloxGroup,
                "www.roblox.com" => SocialLinkType::RobloxGroup,
                "guilded.gg" => SocialLinkType::Guilded,
                domain => {
                    return Err(format!(
                        "Unknown social link type for domain name {}",
                        domain
                    ))
                }
            };
            resources.push(RobloxResource::new(
                &format!("socialLink_{}", domain),
                RobloxInputs::SocialLink(SocialLinkInputs {
                    title: social_link.title.clone(),
                    url: social_link.url.to_string(),
                    link_type,
                }),
                &[&experience],
            ));
        }
    }

    if let Some(products) = &target_config.products {
        for (name, product) in products {
            let mut product_resource = RobloxResource::new(
                &format!("product_{}", name),
                RobloxInputs::Product(ProductInputs {
                    name: product.name.clone(),
                    description: product.description.clone(),
                    price: product.price.clone(),
                }),
                &[&experience],
            );

            if let Some(icon_path) = &product.icon {
                let icon_resource = RobloxResource::new(
                    &format!("productIcon_{}", name),
                    RobloxInputs::ProductIcon(FileInputs {
                        file_path: icon_path.clone(),
                        file_hash: get_file_hash(project_path.join(icon_path))?,
                    }),
                    &[&experience],
                );
                product_resource.add_dependency(&icon_resource);
                resources.push(icon_resource);
            }

            resources.push(product_resource);
        }
    }

    if let Some(passes) = &target_config.passes {
        for (name, pass) in passes {
            let pass_resource = RobloxResource::new(
                &format!("pass_{}", name),
                RobloxInputs::Pass(PassInputs {
                    name: pass.name.clone(),
                    description: pass.description.clone(),
                    price: pass.price.clone(),
                    icon_file_path: pass.icon.clone(),
                }),
                &[&experience],
            );
            resources.push(RobloxResource::new(
                &format!("passIcon_{}", name),
                RobloxInputs::PassIcon(FileInputs {
                    file_path: pass.icon.clone(),
                    file_hash: get_file_hash(project_path.join(pass.icon.clone()))?,
                }),
                &[&pass_resource],
            ));
            resources.push(pass_resource);
        }
    }

    if let Some(badges) = &target_config.badges {
        for (name, badge) in badges {
            let badge_resource = RobloxResource::new(
                &format!("badge_{}", name),
                RobloxInputs::Badge(BadgeInputs {
                    name: badge.name.clone(),
                    description: badge.description.clone(),
                    enabled: badge.enabled.unwrap_or(true),
                    icon_file_path: badge.icon.clone(),
                }),
                &[&experience],
            );
            resources.push(RobloxResource::new(
                &format!("badgeIcon_{}", name),
                RobloxInputs::BadgeIcon(FileInputs {
                    file_path: badge.icon.clone(),
                    file_hash: get_file_hash(project_path.join(badge.icon.clone()))?,
                }),
                &[&badge_resource],
            ));
            resources.push(badge_resource);
        }
    }

    if let Some(assets) = &target_config.assets {
        for asset_config in assets {
            let assets = match asset_config.clone() {
                AssetTargetConfig::File(file) => {
                    let relative_to_project = project_path.join(file.clone());
                    let relative_to_project = relative_to_project
                        .to_str()
                        .ok_or(format!("Path was invalid: {}", file))?;
                    let paths = glob::glob(relative_to_project)
                        .map_err(|e| format!("Glob pattern invalid: {}", e))?;

                    let mut assets = Vec::new();
                    for path in paths {
                        let path = path.map_err(|e| format!("Glob pattern invalid: {}", e))?;
                        let name = path
                            .file_stem()
                            .map(OsStr::to_str)
                            .flatten()
                            .ok_or(format!("Asset path is not a file: {}", path.display()))?
                            .to_owned();

                        let relative_file = path.canonicalize();
                        let relative_file =
                            relative_file.map_err(|e| format!("Failed to canonizalize: {}", e))?;
                        let relative_file = relative_file
                            .strip_prefix(
                                project_path
                                    .canonicalize()
                                    .map_err(|e| format!("Failed to canonizalize: {}", e))?,
                            )
                            .map_err(|e| format!("Failed to relativize path: {}", e))?
                            .to_str()
                            .ok_or(format!("Path was invalid: {}", path.display()))?;

                        assets.push((relative_file.to_owned(), name));
                    }
                    assets
                }
                AssetTargetConfig::FileWithAlias { file, name } => vec![(file, name)],
            };

            for (file, alias) in assets {
                let resource_inputs = match Path::new(&file).extension().map(OsStr::to_str) {
                    Some(Some("bmp" | "gif" | "jpeg" | "jpg" | "png" | "tga")) => {
                        RobloxInputs::ImageAsset(FileWithGroupIdInputs {
                            file_path: file.clone(),
                            file_hash: get_file_hash(project_path.join(&file))?,
                            group_id,
                        })
                    }
                    Some(Some("ogg" | "mp3")) => RobloxInputs::AudioAsset(FileWithGroupIdInputs {
                        file_path: file.clone(),
                        file_hash: get_file_hash(project_path.join(&file))?,
                        group_id,
                    }),
                    _ => return Err(format!("Unable to determine asset type for file: {}", file)),
                };

                let alias_folder = match resource_inputs {
                    RobloxInputs::ImageAsset(_) => "Images",
                    RobloxInputs::AudioAsset(_) => "Audio",
                    _ => unreachable!(),
                };

                let asset_resource =
                    RobloxResource::new(&format!("asset_{}", file), resource_inputs, &[]);
                resources.push(RobloxResource::new(
                    &format!("assetAlias_{}", file),
                    RobloxInputs::AssetAlias(AssetAliasInputs {
                        name: format!("{}/{}", alias_folder, alias),
                    }),
                    &[&experience, &asset_resource],
                ));
                resources.push(asset_resource);
            }
        }
    }

    Ok(ResourceGraph::new(&resources))
}

pub fn get_desired_graph(
    project_path: &Path,
    target_config: &TargetConfig,
    owner_config: &OwnerConfig,
) -> Result<ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>, String> {
    match target_config {
        TargetConfig::Experience(experience_target_config) => {
            get_desired_experience_graph(project_path, experience_target_config, owner_config)
        }
    }
}

pub fn import_graph(
    roblox_api: &mut RobloxApi,
    experience_id: AssetId,
) -> Result<ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>, String> {
    unimplemented!()
}

pub async fn save_state_to_remote(config: &RemoteStateConfig, data: &[u8]) -> Result<(), String> {
    logger::log(format!("Saving to remote object {}", Paint::cyan(config)));

    let client = S3Client::new(config.region.clone());
    let res = client
        .put_object(rusoto_s3::PutObjectRequest {
            bucket: config.bucket.clone(),
            key: format!("{}.mantle-state.yml", config.key),
            body: Some(rusoto_core::ByteStream::from(data.to_vec())),
            ..Default::default()
        })
        .await;

    res.map(|_| ())
        .map_err(|e| format!("Failed to save state to remote: {}", e))
}

pub fn save_state_to_file(project_path: &Path, data: &[u8]) -> Result<(), String> {
    let state_file_path = get_state_file_path(project_path);

    logger::log(format!(
        "Saving to local file {}. It is recommended you commit this file to your source control",
        Paint::cyan(state_file_path.display())
    ));

    fs::write(&state_file_path, data).map_err(|e| {
        format!(
            "Unable to write state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    Ok(())
}

pub async fn save_state(
    project_path: &Path,
    state_config: &StateConfig,
    state: &ResourceStateVLatest,
) -> Result<(), String> {
    let data = serde_yaml::to_vec(&ResourceState::Versioned(VersionedResourceState::V3(
        state.to_owned(),
    )))
    .map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    match state_config {
        StateConfig::Local => save_state_to_file(project_path, &data),
        StateConfig::Remote(config) => save_state_to_remote(config, &data).await,
    }
}
