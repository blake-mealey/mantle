mod aws_credentials_provider;
mod legacy_resources;
pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use chrono::Utc;
use clap::crate_version;
use rbx_api::{
    experiences::models::GetExperienceResponse,
    models::{AssetId, CreatorType},
    social_links::models::SocialLinkType,
    RobloxApi,
};
use rusoto_core::{HttpClient, Region};
use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use yansi::Paint;

use super::{
    config::{
        AssetTargetConfig, Config, EnvironmentConfig, ExperienceTargetConfig, OwnerConfig,
        PlayabilityTargetConfig, RemoteStateConfig, StateConfig, TargetConfig,
    },
    resource_graph::ResourceGraph,
    roblox_resource_manager::*,
};

use self::{
    aws_credentials_provider::AwsCredentialsProvider, v1::ResourceStateV1, v2::ResourceStateV2,
    v3::ResourceStateV3, v4::ResourceStateV4, v5::ResourceStateV5,
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
    #[serde(rename = "4")]
    V4(ResourceStateV4),
    #[serde(rename = "5")]
    V5(ResourceStateV5),
}

pub type ResourceStateVLatest = ResourceStateV5;

fn get_state_file_path(project_path: &Path, key: Option<&str>) -> PathBuf {
    project_path.join(format!("{}.mantle-state.yml", key.unwrap_or_default()))
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

fn get_state_from_file(
    project_path: &Path,
    key: Option<&str>,
) -> Result<Option<ResourceState>, String> {
    let state_file_path = get_state_file_path(project_path, key);
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

fn create_client(region: Region) -> S3Client {
    S3Client::new_with(
        HttpClient::new().unwrap(),
        AwsCredentialsProvider::new(),
        region,
    )
}

async fn get_state_from_remote(
    config: &RemoteStateConfig,
) -> Result<Option<ResourceState>, String> {
    logger::log(format!(
        "Loading previous state from remote object {}",
        Paint::cyan(config)
    ));

    let client = create_client(config.region.clone());
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

pub async fn get_state_from_source(
    project_path: &Path,
    source: StateConfig,
) -> Result<ResourceStateVLatest, String> {
    let state = match source {
        StateConfig::Local => get_state_from_file(project_path, None)?,
        StateConfig::LocalKey(key) => get_state_from_file(project_path, Some(&key))?,
        StateConfig::Remote(config) => get_state_from_remote(&config).await?,
    };

    // Migrate previous state formats
    Ok(match state {
        Some(ResourceState::Unversioned(state)) => ResourceStateV5::from(ResourceStateV4::from(
            ResourceStateV3::from(ResourceStateV2::from(state)),
        )),
        Some(ResourceState::Versioned(VersionedResourceState::V1(state))) => ResourceStateV5::from(
            ResourceStateV4::from(ResourceStateV3::from(ResourceStateV2::from(state))),
        ),
        Some(ResourceState::Versioned(VersionedResourceState::V2(state))) => {
            ResourceStateV5::from(ResourceStateV4::from(ResourceStateV3::from(state)))
        }
        Some(ResourceState::Versioned(VersionedResourceState::V3(state))) => {
            ResourceStateV5::from(ResourceStateV4::from(state))
        }
        Some(ResourceState::Versioned(VersionedResourceState::V4(state))) => {
            ResourceStateV5::from(state)
        }
        Some(ResourceState::Versioned(VersionedResourceState::V5(state))) => state,
        None => ResourceStateVLatest {
            environments: HashMap::new(),
        },
    })
}

pub async fn get_state(
    project_path: &Path,
    config: &Config,
) -> Result<ResourceStateVLatest, String> {
    get_state_from_source(project_path, config.state.clone()).await
}

pub async fn get_previous_state(
    project_path: &Path,
    config: &Config,
    environment_config: &EnvironmentConfig,
) -> Result<ResourceStateVLatest, String> {
    let mut state = get_state(project_path, config).await?;

    if state.environments.get(&environment_config.label).is_none() {
        logger::log(format!(
            "No previous state for environment {}",
            Paint::cyan(environment_config.label.clone())
        ));
        state
            .environments
            .insert(environment_config.label.clone(), Vec::new());
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
    }

    if let Some(places) = &target_config.places {
        if !places.contains_key("start") {
            return Err("No start place specified".to_owned());
        }

        for (label, place) in places.iter() {
            let place_resource = RobloxResource::new(
                &format!("place_{}", label),
                RobloxInputs::Place(PlaceInputs {
                    is_start: label == "start",
                }),
                &[&experience],
            );
            resources.push(place_resource.clone());

            if let Some(file) = &place.file {
                resources.push(RobloxResource::new(
                    &format!("placeFile_{}", label),
                    RobloxInputs::PlaceFile(FileInputs {
                        file_path: file.clone(),
                        file_hash: get_file_hash(project_path.join(file))?,
                    }),
                    &[&place_resource],
                ));
            }

            if let Some(configuration) = &place.configuration {
                resources.push(RobloxResource::new(
                    &format!("placeConfiguration_{}", label),
                    RobloxInputs::PlaceConfiguration(configuration.clone().into()),
                    &[&place_resource],
                ));
            }
        }
    } else {
        return Err("No start place specified".to_owned());
    }

    if let Some(icon_path) = &target_config.icon {
        resources.push(RobloxResource::new(
            "experienceIcon_singleton",
            RobloxInputs::ExperienceIcon(FileInputs {
                file_path: icon_path.clone(),
                file_hash: get_file_hash(project_path.join(icon_path))?,
            }),
            &[&experience],
        ));
    }

    if let Some(thumbnails) = &target_config.thumbnails {
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
        for (label, product) in products {
            let product_resource = RobloxResource::new(
                &format!("product_{}", label),
                RobloxInputs::Product(ProductInputs {
                    name: product.name.clone(),
                    description: product.description.clone().unwrap_or_default(),
                    price: product.price,
                }),
                &[&experience],
            );

            if let Some(icon_path) = &product.icon {
                resources.push(RobloxResource::new(
                    &format!("productIcon_{}", label),
                    RobloxInputs::ProductIcon(FileInputs {
                        file_path: icon_path.clone(),
                        file_hash: get_file_hash(project_path.join(icon_path))?,
                    }),
                    &[&product_resource],
                ));
            }

            resources.push(product_resource);
        }
    }

    if let Some(passes) = &target_config.passes {
        for (label, pass) in passes {
            resources.push(RobloxResource::new(
                &format!("pass_{}", label),
                RobloxInputs::Pass(PassInputs {
                    name: pass.name.clone(),
                    description: pass.description.clone().unwrap_or_default(),
                    price: pass.price,
                    icon_file_path: pass.icon.clone(),
                    icon_file_hash: get_file_hash(project_path.join(pass.icon.clone()))?,
                }),
                &[&experience],
            ));
        }
    }

    if let Some(badges) = &target_config.badges {
        for (label, badge) in badges {
            let badge_resource = RobloxResource::new(
                &format!("badge_{}", label),
                RobloxInputs::Badge(BadgeInputs {
                    name: badge.name.clone(),
                    description: badge.description.clone().unwrap_or_default(),
                    enabled: badge.enabled.unwrap_or(true),
                    icon_file_path: badge.icon.clone(),
                }),
                &[&experience],
            );
            resources.push(RobloxResource::new(
                &format!("badgeIcon_{}", label),
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
                            .and_then(OsStr::to_str)
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

    if let Some(spatial_voice) = &target_config.spatial_voice {
        resources.push(RobloxResource::new(
            "spatialVoice_singleton",
            RobloxInputs::SpatialVoice(SpatialVoiceInputs {
                enabled: spatial_voice.enabled,
            }),
            &[&experience],
        ));
    }

    if let Some(notifications) = &target_config.notifications {
        for (label, notification) in notifications {
            let name = match &notification.name {
                Some(name) => name.clone(),
                None => label.clone(),
            };

            resources.push(RobloxResource::new(
                &format!("notification_{}", label),
                RobloxInputs::Notification(NotificationInputs {
                    name,
                    content: notification.content.to_string(),
                }),
                &[&experience],
            ));
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

pub async fn import_graph(
    roblox_api: &RobloxApi,
    target_id: AssetId,
) -> Result<ResourceGraph<RobloxResource, RobloxInputs, RobloxOutputs>, String> {
    let mut resources: Vec<RobloxResource> = Vec::new();

    logger::log("Importing experience");
    let GetExperienceResponse {
        root_place_id: start_place_id,
        is_active: is_experience_active,
        creator_target_id,
        creator_type,
    } = roblox_api.get_experience(target_id).await?;

    let group_id = match creator_type {
        CreatorType::User => None,
        CreatorType::Group => Some(creator_target_id),
    };

    let experience = RobloxResource::existing(
        "experience_singleton",
        RobloxInputs::Experience(ExperienceInputs { group_id }),
        RobloxOutputs::Experience(ExperienceOutputs {
            asset_id: target_id,
            start_place_id,
        }),
        &[],
    );
    resources.push(experience.clone());

    resources.push(RobloxResource::existing(
        "experienceActivation_singleton",
        RobloxInputs::ExperienceActivation(ExperienceActivationInputs {
            is_active: is_experience_active,
        }),
        RobloxOutputs::ExperienceActivation,
        &[&experience],
    ));

    logger::log("Importing experience configuration");
    let experience_configuration = roblox_api.get_experience_configuration(target_id).await?;
    resources.push(RobloxResource::existing(
        "experienceConfiguration_singleton",
        RobloxInputs::ExperienceConfiguration(experience_configuration),
        RobloxOutputs::ExperienceConfiguration,
        &[&experience],
    ));

    // We intentionally do not import the game icon because we do not know of an API which returns
    // the correct ID for it to be removed.

    logger::log("Importing experience thumbnails");
    let thumbnails = roblox_api.get_experience_thumbnails(target_id).await?;
    let mut thumbnail_resources: Vec<RobloxResource> = Vec::new();
    for thumbnail in thumbnails {
        thumbnail_resources.push(RobloxResource::existing(
            &format!("experienceThumbnail_{}", thumbnail.id),
            RobloxInputs::ExperienceThumbnail(FileInputs {
                file_path: "fake-path".to_owned(),
                file_hash: "fake-hash".to_owned(),
            }),
            RobloxOutputs::ExperienceThumbnail(AssetOutputs {
                asset_id: thumbnail.id,
            }),
            &[&experience],
        ));
    }
    let mut thumbnail_order_dependencies: Vec<&RobloxResource> =
        thumbnail_resources.iter().collect();
    thumbnail_order_dependencies.push(&experience);
    resources.push(RobloxResource::existing(
        "experienceThumbnailOrder_singleton",
        RobloxInputs::ExperienceThumbnailOrder,
        RobloxOutputs::ExperienceThumbnailOrder,
        &thumbnail_order_dependencies,
    ));
    resources.extend(thumbnail_resources);

    logger::log("Importing places");
    let places = roblox_api.get_all_places(target_id).await?;
    for place in places {
        let resource_id = if place.is_root_place {
            "start".to_owned()
        } else {
            place.id.to_string()
        };

        let place_resource = RobloxResource::existing(
            &format!("place_{}", resource_id),
            RobloxInputs::Place(PlaceInputs {
                is_start: place.is_root_place,
            }),
            RobloxOutputs::Place(AssetOutputs { asset_id: place.id }),
            &[&experience],
        );
        resources.push(place_resource.clone());

        resources.push(RobloxResource::existing(
            &format!("placeFile_{}", resource_id),
            RobloxInputs::PlaceFile(FileInputs {
                file_path: "fake-path".to_owned(),
                file_hash: "fake-hash".to_owned(),
            }),
            RobloxOutputs::PlaceFile(PlaceFileOutputs {
                version: place.current_saved_version,
            }),
            &[&place_resource],
        ));

        resources.push(RobloxResource::existing(
            &format!("placeConfiguration_{}", resource_id),
            RobloxInputs::PlaceConfiguration(place.into()),
            RobloxOutputs::PlaceConfiguration,
            &[&place_resource],
        ));
    }

    logger::log("Importing social links");
    let social_links = roblox_api.list_social_links(target_id).await?;
    for social_link in social_links {
        let domain = social_link
            .url
            .domain()
            .ok_or_else(|| "Invalid social link URL".to_owned())?;
        resources.push(RobloxResource::existing(
            &format!("socialLink_{}", domain),
            RobloxInputs::SocialLink(SocialLinkInputs {
                title: social_link.title,
                url: social_link.url.to_string(),
                link_type: social_link.link_type,
            }),
            RobloxOutputs::SocialLink(AssetOutputs {
                asset_id: social_link.id,
            }),
            &[&experience],
        ));
    }

    logger::log("Importing products");
    let developer_products = roblox_api.get_all_developer_products(target_id).await?;
    for product in developer_products {
        let product_resource = RobloxResource::existing(
            &format!("product_{}", product.product_id),
            RobloxInputs::Product(ProductInputs {
                name: product.name,
                description: product.description.unwrap_or_default(),
                price: product.price_in_robux,
            }),
            RobloxOutputs::Product(ProductOutputs {
                asset_id: product.product_id,
                product_id: product.developer_product_id,
            }),
            &[&experience],
        );
        if let Some(icon_id) = product.icon_image_asset_id {
            resources.push(RobloxResource::existing(
                &format!("productIcon_{}", product.product_id),
                RobloxInputs::ProductIcon(FileInputs {
                    file_path: "fake-path".to_owned(),
                    file_hash: "fake-hash".to_owned(),
                }),
                RobloxOutputs::ProductIcon(AssetOutputs { asset_id: icon_id }),
                &[&product_resource],
            ));
        }
        resources.push(product_resource);
    }

    logger::log("Importing passes");
    let game_passes = roblox_api.get_all_game_passes(target_id).await?;
    for pass in game_passes {
        resources.push(RobloxResource::existing(
            &format!("pass_{}", pass.target_id),
            RobloxInputs::Pass(PassInputs {
                name: pass.name,
                description: pass.description,
                price: pass.price_in_robux,
                icon_file_path: "fake-path".to_owned(),
                icon_file_hash: "fake-hash".to_owned(),
            }),
            RobloxOutputs::Pass(PassOutputs {
                asset_id: pass.target_id,
                icon_asset_id: pass.icon_image_asset_id,
            }),
            &[&experience],
        ));
    }

    logger::log("Importing badges");
    let badges = roblox_api.get_all_badges(target_id).await?;
    for badge in badges {
        let badge_resource = RobloxResource::existing(
            &format!("badge_{}", badge.id),
            RobloxInputs::Badge(BadgeInputs {
                name: badge.name,
                description: badge.description,
                enabled: badge.enabled,
                icon_file_path: "fake-path".to_owned(),
            }),
            RobloxOutputs::Badge(AssetWithInitialIconOutputs {
                asset_id: badge.id,
                initial_icon_asset_id: badge.icon_image_id,
            }),
            &[&experience],
        );
        resources.push(RobloxResource::existing(
            &format!("badgeIcon_{}", badge.id),
            RobloxInputs::BadgeIcon(FileInputs {
                file_path: "fake-path".to_owned(),
                file_hash: "fake-hash".to_owned(),
            }),
            RobloxOutputs::BadgeIcon(AssetOutputs {
                asset_id: badge.icon_image_id,
            }),
            &[&badge_resource],
        ));
        resources.push(badge_resource);
    }

    logger::log("Importing assets");
    let assets = roblox_api.get_all_asset_aliases(target_id).await?;
    for asset in assets {
        let resource_data = match asset.asset.type_id {
            1 => Some((
                RobloxInputs::ImageAsset(FileWithGroupIdInputs {
                    file_path: "fake-path".to_owned(),
                    file_hash: "fake-hash".to_owned(),
                    group_id,
                }),
                RobloxOutputs::ImageAsset(ImageAssetOutputs {
                    asset_id: asset.target_id,
                    decal_asset_id: None,
                }),
            )),
            3 => Some((
                RobloxInputs::AudioAsset(FileWithGroupIdInputs {
                    file_path: "fake-path".to_owned(),
                    file_hash: "fake-hash".to_owned(),
                    group_id,
                }),
                RobloxOutputs::AudioAsset(AssetOutputs {
                    asset_id: asset.target_id,
                }),
            )),
            _ => None,
        };

        if let Some((resource_inputs, resource_outputs)) = resource_data {
            let asset_resource = RobloxResource::existing(
                &format!("asset_{}", asset.name),
                resource_inputs,
                resource_outputs,
                &[],
            );
            resources.push(RobloxResource::existing(
                &format!("assetAlias_{}", asset.name),
                RobloxInputs::AssetAlias(AssetAliasInputs {
                    name: asset.name.clone(),
                }),
                RobloxOutputs::AssetAlias(AssetAliasOutputs { name: asset.name }),
                &[&experience, &asset_resource],
            ));
            resources.push(asset_resource);
        }
    }

    logger::log("Importing spatial voice settings");
    let spatial_voice = roblox_api.get_spatial_voice_settings(target_id).await?;
    // only add the resource if it was enabled since the default is disabled
    if spatial_voice.is_universe_enabled_for_voice {
        resources.push(RobloxResource::existing(
            "spatialVoice_singleton",
            RobloxInputs::SpatialVoice(SpatialVoiceInputs {
                enabled: spatial_voice.is_universe_enabled_for_voice,
            }),
            RobloxOutputs::SpatialVoice,
            &[&experience],
        ));
    }

    logger::log("Importing notifications");
    let notifications = roblox_api.get_all_notifications(target_id).await?;
    for notification in notifications {
        resources.push(RobloxResource::existing(
            &format!("notification_{}", notification.name),
            RobloxInputs::Notification(NotificationInputs {
                name: notification.name,
                content: notification.content,
            }),
            RobloxOutputs::Notification(NotificationOutputs {
                asset_id: notification.id,
            }),
            &[&experience],
        ));
    }

    Ok(ResourceGraph::new(&resources))
}

pub async fn save_state_to_remote(config: &RemoteStateConfig, data: &[u8]) -> Result<(), String> {
    logger::log(format!("Saving to remote object {}", Paint::cyan(config)));

    let client = create_client(config.region.clone());
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

pub fn save_state_to_file(
    project_path: &Path,
    data: &[u8],
    file_path: Option<&str>,
) -> Result<(), String> {
    let state_file_path = get_state_file_path(project_path, file_path);

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

fn serialize_state(state: &ResourceStateVLatest) -> Result<Vec<u8>, String> {
    let utc = Utc::now();
    let mut data = format!("#\n\
                                   # WARNING - Generated file. Do not modify directly unless you know \
                                     what you are doing!\n\
                                   # This file was generated by Mantle v{} on {}\n\
                                   #\n\n",
                                crate_version!(),
                                utc.format("%FT%TZ")
                            ).as_bytes().to_vec();

    let state_data = serde_yaml::to_vec(&ResourceState::Versioned(VersionedResourceState::V5(
        state.to_owned(),
    )))
    .map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    data.extend(state_data);

    Ok(data)
}

pub async fn save_state(
    project_path: &Path,
    state_config: &StateConfig,
    state: &ResourceStateVLatest,
) -> Result<(), String> {
    let data = serialize_state(state)?;

    match state_config {
        StateConfig::Local => save_state_to_file(project_path, &data, None),
        StateConfig::LocalKey(key) => save_state_to_file(project_path, &data, Some(key)),
        StateConfig::Remote(config) => save_state_to_remote(config, &data).await,
    }
}
