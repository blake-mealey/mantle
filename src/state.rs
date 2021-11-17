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
    resource_manager::{
        resource_types, AssetAliasOutputs, AssetId, AudioAssetOutputs, BadgeIconOutputs,
        BadgeOutputs, ExperienceDeveloperProductIconOutputs, ExperienceDeveloperProductOutputs,
        ExperienceOutputs, ExperienceThumbnailOutputs, GamePassIconOutputs, GamePassOutputs,
        ImageAssetOutputs, PlaceFileOutputs, PlaceOutputs, SINGLETON_RESOURCE_ID,
    },
    resources::{Input, InputRef, Resource, ResourceGraph},
    roblox_api::{
        CreatorType, ExperienceConfigurationModel, GetExperienceResponse, PlaceConfigurationModel,
        RobloxApi,
    },
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV2 {
    pub environments: HashMap<String, Vec<Resource>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV1 {
    pub deployments: HashMap<String, Vec<Resource>>,
}
impl From<ResourceStateV1> for ResourceStateV2 {
    fn from(state: ResourceStateV1) -> Self {
        // State format change: "deployments" -> "environments"
        let mut environments = state.deployments;
        for (_, resources) in environments.iter_mut() {
            for resource in resources {
                let r_type = resource.resource_type.as_str();

                // Resources format change: remove assetId input from experience and place resources
                // to avoid unnecessary recreation of resources
                if matches!(r_type, resource_types::EXPERIENCE | resource_types::PLACE) {
                    resource.inputs.remove("assetId");
                }

                // Resources format change: add groupId input to experience and asset resources to
                // avoid unnecessary recreation of resources
                if matches!(
                    r_type,
                    resource_types::EXPERIENCE
                        | resource_types::IMAGE_ASSET
                        | resource_types::AUDIO_ASSET
                ) {
                    resource
                        .inputs
                        .insert("groupId".to_owned(), Input::Value(serde_yaml::Value::Null));
                }
            }
        }

        ResourceStateV2 { environments }
    }
}

fn get_state_file_path(project_path: &Path) -> PathBuf {
    project_path.join(".mantle-state.yml")
}

fn get_hash(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    format!("{:x}", digest)
}

fn get_file_hash(file_path: &Path) -> Result<String, String> {
    let buffer = fs::read(file_path).map_err(|e| {
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
) -> Result<ResourceStateV2, String> {
    let state = match config.state.clone() {
        StateConfig::Local => get_state_from_file(project_path)?,
        StateConfig::Remote(config) => get_state_from_remote(&config).await?,
    };

    let mut state = match state {
        Some(ResourceState::Unversioned(state)) => state.into(),
        Some(ResourceState::Versioned(VersionedResourceState::V1(state))) => state.into(),
        Some(ResourceState::Versioned(VersionedResourceState::V2(state))) => state,
        None => ResourceStateV2 {
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
) -> Result<ResourceGraph, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let group_id = match owner_config {
        OwnerConfig::Personal => None,
        OwnerConfig::Group(group_id) => Some(*group_id),
    };

    let experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_value_input("groupId", &group_id)?
        .clone();
    let experience_asset_id_ref = experience.get_input_ref("assetId");
    let experience_start_place_id_ref = experience.get_input_ref("startPlaceId");
    resources.push(experience);

    if let Some(experience_configuration) = &target_config.configuration {
        resources.push(
            Resource::new(
                resource_types::EXPERIENCE_CONFIGURATION,
                SINGLETON_RESOURCE_ID,
            )
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input::<ExperienceConfigurationModel>(
                "configuration",
                &experience_configuration.into(),
            )?
            .clone(),
        );

        resources.push(
            Resource::new(resource_types::EXPERIENCE_ACTIVATION, SINGLETON_RESOURCE_ID)
                .add_value_input(
                    "isActive",
                    &!matches!(
                        experience_configuration.playability,
                        Some(PlayabilityTargetConfig::Private)
                    ),
                )?
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .clone(),
        );

        if let Some(file_path) = &experience_configuration.icon {
            resources.push(
                Resource::new(resource_types::EXPERIENCE_ICON, file_path)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_ref_input("startPlaceId", &experience_start_place_id_ref)
                    .add_value_input("filePath", file_path)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(file_path).as_path())?,
                    )?
                    .clone(),
            );
        }

        if let Some(thumbnails) = &experience_configuration.thumbnails {
            let mut thumbnail_asset_id_refs: Vec<InputRef> = Vec::new();
            for file_path in thumbnails {
                let thumbnail_resource =
                    Resource::new(resource_types::EXPERIENCE_THUMBNAIL, file_path)
                        .add_ref_input("experienceId", &experience_asset_id_ref)
                        .add_value_input("filePath", file_path)?
                        .add_value_input(
                            "fileHash",
                            &get_file_hash(project_path.join(file_path).as_path())?,
                        )?
                        .clone();
                thumbnail_asset_id_refs.push(thumbnail_resource.get_input_ref("assetId"));
                resources.push(thumbnail_resource);
            }
            resources.push(
                Resource::new(
                    resource_types::EXPERIENCE_THUMBNAIL_ORDER,
                    SINGLETON_RESOURCE_ID,
                )
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_ref_input_list("assetIds", &thumbnail_asset_id_refs)
                .clone(),
            );
        }
    }

    if let Some(places) = &target_config.places {
        if !places.keys().any(|n| n == "start") {
            return Err("No start place defined".to_owned());
        }

        for (name, place) in places.iter() {
            let place_resource = Resource::new(resource_types::PLACE, name)
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_ref_input("startPlaceId", &experience_start_place_id_ref)
                .add_value_input("isStart", &(name == "start"))?
                .clone();
            let place_asset_id_ref = place_resource.get_input_ref("assetId");
            resources.push(place_resource);

            if let Some(file) = &place.file {
                resources.push(
                    Resource::new(resource_types::PLACE_FILE, name)
                        .add_ref_input("assetId", &place_asset_id_ref)
                        .add_value_input("filePath", file)?
                        .add_value_input(
                            "fileHash",
                            &get_file_hash(project_path.join(file).as_path())?,
                        )?
                        .clone(),
                );
            }

            if let Some(configuration) = &place.configuration {
                resources.push(
                    Resource::new(resource_types::PLACE_CONFIGURATION, name)
                        .add_ref_input("assetId", &place_asset_id_ref)
                        .add_value_input::<PlaceConfigurationModel>(
                            "configuration",
                            &configuration.clone().into(),
                        )?
                        .clone(),
                );
            }
        }
    } else {
        return Err("No start place defined".to_owned());
    }

    if let Some(products) = &target_config.products {
        for (name, product) in products {
            let mut product_resource = Resource::new(resource_types::DEVELOPER_PRODUCT, name)
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_value_input("name", &product.name)?
                .add_value_input("price", &product.price)?
                .add_value_input(
                    "description",
                    product.description.as_ref().unwrap_or(&"".to_owned()),
                )?
                .clone();
            if let Some(icon_path) = &product.icon {
                let icon_resource = Resource::new(resource_types::DEVELOPER_PRODUCT_ICON, name)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_value_input("filePath", icon_path)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(icon_path).as_path())?,
                    )?
                    .clone();
                resources.push(icon_resource.clone());
                product_resource = product_resource
                    .add_ref_input("iconAssetId", &icon_resource.get_input_ref("assetId"))
                    .clone();
            }
            resources.push(product_resource);
        }
    }

    if let Some(passes) = &target_config.passes {
        for (name, pass) in passes {
            let pass_resource = Resource::new(resource_types::GAME_PASS, name)
                .add_ref_input("startPlaceId", &experience_start_place_id_ref)
                .add_value_input("name", &pass.name)?
                .add_value_input("description", &pass.description)?
                .add_value_input("price", &pass.price)?
                .add_value_input("iconFilePath", &pass.icon)?
                .clone();
            resources.push(pass_resource.clone());
            resources.push(
                Resource::new(resource_types::GAME_PASS_ICON, name)
                    .add_ref_input("gamePassId", &pass_resource.get_input_ref("assetId"))
                    .add_ref_input(
                        "initialAssetId",
                        &pass_resource.get_input_ref("initialIconAssetId"),
                    )
                    .add_value_input("filePath", &pass.icon)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(&pass.icon).as_path())?,
                    )?
                    .clone(),
            )
        }
    }

    if let Some(badges) = &target_config.badges {
        for (name, badge) in badges {
            let badge_resource = Resource::new(resource_types::BADGE, name)
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_value_input("name", &badge.name)?
                .add_value_input("description", &badge.description)?
                .add_value_input("enabled", &badge.enabled.unwrap_or(true))?
                .add_value_input("iconFilePath", &badge.icon)?
                .clone();
            resources.push(badge_resource.clone());
            resources.push(
                Resource::new(resource_types::BADGE_ICON, name)
                    .add_ref_input("badgeId", &badge_resource.get_input_ref("assetId"))
                    .add_ref_input(
                        "initialAssetId",
                        &badge_resource.get_input_ref("initialIconAssetId"),
                    )
                    .add_value_input("filePath", &badge.icon)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(&badge.icon).as_path())?,
                    )?
                    .clone(),
            )
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
                let resource_type = match Path::new(&file).extension().map(OsStr::to_str) {
                    Some(Some("bmp" | "gif" | "jpeg" | "jpg" | "png" | "tga")) => {
                        resource_types::IMAGE_ASSET
                    }
                    Some(Some("ogg" | "mp3")) => resource_types::AUDIO_ASSET,
                    _ => return Err(format!("Unable to determine asset type for file: {}", file)),
                };

                let alias_folder = match resource_type {
                    resource_types::IMAGE_ASSET => "Images",
                    resource_types::AUDIO_ASSET => "Audio",
                    _ => unreachable!(),
                };

                let asset_resource = Resource::new(resource_type, &file)
                    .add_value_input("filePath", &file)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(&file).as_path())?,
                    )?
                    .add_value_input("groupId", &group_id)?
                    .clone();
                resources.push(asset_resource.clone());
                resources.push(
                    Resource::new(resource_types::ASSET_ALIAS, &file)
                        .add_ref_input("experienceId", &experience_asset_id_ref)
                        .add_ref_input("assetId", &asset_resource.get_input_ref("assetId"))
                        .add_value_input("name", &format!("{}/{}", alias_folder, alias))?
                        .clone(),
                )
            }
        }
    }

    Ok(ResourceGraph::new(&resources))
}

pub fn get_desired_graph(
    project_path: &Path,
    target_config: &TargetConfig,
    owner_config: &OwnerConfig,
) -> Result<ResourceGraph, String> {
    match target_config {
        TargetConfig::Experience(experience_target_config) => {
            get_desired_experience_graph(project_path, experience_target_config, owner_config)
        }
    }
}

pub fn import_graph(
    roblox_api: &mut RobloxApi,
    experience_id: AssetId,
) -> Result<ResourceGraph, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let GetExperienceResponse {
        root_place_id: start_place_id,
        is_active: is_experience_active,
        creator_target_id,
        creator_type,
    } = roblox_api.get_experience(experience_id)?;

    let group_id = match creator_type {
        CreatorType::User => None,
        CreatorType::Group => Some(creator_target_id),
    };

    let experience_resource = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_value_input("groupId", &group_id)?
        .set_outputs(ExperienceOutputs {
            asset_id: experience_id,
            start_place_id,
        })?
        .clone();
    let experience_asset_id_ref = experience_resource.get_input_ref("assetId");
    let experience_start_place_id_ref = experience_resource.get_input_ref("startPlaceId");
    resources.push(experience_resource);

    resources.push(
        Resource::new(resource_types::EXPERIENCE_ACTIVATION, SINGLETON_RESOURCE_ID)
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input("isActive", &is_experience_active)?
            .clone(),
    );

    let experience_configuration = roblox_api.get_experience_configuration(experience_id)?;
    resources.push(
        Resource::new(
            resource_types::EXPERIENCE_CONFIGURATION,
            SINGLETON_RESOURCE_ID,
        )
        .add_ref_input("experienceId", &experience_asset_id_ref)
        .add_value_input("configuration", &experience_configuration)?
        .clone(),
    );

    // We intentionally do not import the game icon because we do not know of an API which returns
    // the correct ID for it to be removed.

    let thumbnails = roblox_api.get_experience_thumbnails(experience_id)?;
    let mut thumbnail_asset_id_refs: Vec<InputRef> = Vec::new();
    for thumbnail in thumbnails {
        let thumbnail_resource = Resource::new(
            resource_types::EXPERIENCE_THUMBNAIL,
            &thumbnail.id.to_string(),
        )
        .add_ref_input("experienceId", &experience_asset_id_ref)
        // TODO: should we get legit values? e.g. pass the URL and its real hash?
        .add_value_input("filePath", &"fake-path")?
        .add_value_input("fileHash", &"fake-hash")?
        .set_outputs(ExperienceThumbnailOutputs {
            asset_id: thumbnail.id,
        })?
        .clone();
        thumbnail_asset_id_refs.push(thumbnail_resource.get_input_ref("assetId"));
        resources.push(thumbnail_resource);
    }
    resources.push(
        Resource::new(
            resource_types::EXPERIENCE_THUMBNAIL_ORDER,
            SINGLETON_RESOURCE_ID,
        )
        .add_ref_input("experienceId", &experience_asset_id_ref)
        .add_ref_input_list("assetIds", &thumbnail_asset_id_refs)
        .clone(),
    );

    let places = roblox_api.get_all_places(experience_id)?;
    for place in places {
        let resource_id = if place.is_root_place {
            "start".to_owned()
        } else {
            place.id.to_string()
        };
        let place_resource = Resource::new(resource_types::PLACE, &resource_id)
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_ref_input("startPlaceId", &experience_start_place_id_ref)
            .add_value_input("isStart", &place.is_root_place)?
            .set_outputs(PlaceOutputs { asset_id: place.id })?
            .clone();
        let place_asset_id_ref = place_resource.get_input_ref("assetId");
        resources.push(place_resource);

        resources.push(
            Resource::new(resource_types::PLACE_FILE, &resource_id)
                .add_ref_input("assetId", &place_asset_id_ref)
                // TODO: should we get legit values?
                .add_value_input("filePath", &"fake_file")?
                .add_value_input("fileHash", &"fake_hash")?
                .set_outputs(PlaceFileOutputs {
                    version: place.current_saved_version,
                })?
                .clone(),
        );

        resources.push(
            Resource::new(resource_types::PLACE_CONFIGURATION, &resource_id)
                .add_ref_input("assetId", &place_asset_id_ref)
                .add_value_input::<PlaceConfigurationModel>("configuration", &place.into())?
                .clone(),
        );
    }

    let developer_products = roblox_api.get_all_developer_products(experience_id)?;
    for product in developer_products {
        let mut product_resource = Resource::new(
            resource_types::DEVELOPER_PRODUCT,
            &product.product_id.to_string(),
        )
        .add_ref_input("experienceId", &experience_asset_id_ref)
        .add_value_input("name", &product.name)?
        .add_value_input("price", &product.price_in_robux)?
        .add_value_input(
            "description",
            product.description.as_ref().unwrap_or(&"".to_owned()),
        )?
        .set_outputs(ExperienceDeveloperProductOutputs {
            asset_id: product.product_id,
            product_id: product.developer_product_id,
        })?
        .clone();
        if let Some(icon_id) = product.icon_image_asset_id {
            let icon_resource = Resource::new(
                resource_types::DEVELOPER_PRODUCT_ICON,
                &product.product_id.to_string(),
            )
            .add_ref_input("experienceId", &experience_asset_id_ref)
            // TODO: should we get legit values? e.g. pass the URL and its real hash?
            .add_value_input("filePath", &"fake-path")?
            .add_value_input("fileHash", &"fake-hash")?
            .set_outputs(ExperienceDeveloperProductIconOutputs { asset_id: icon_id })?
            .clone();
            resources.push(icon_resource.clone());
            product_resource = product_resource
                .add_ref_input("iconAssetId", &icon_resource.get_input_ref("assetId"))
                .clone();
        }
        resources.push(product_resource);
    }

    let game_passes = roblox_api.get_all_game_passes(experience_id)?;
    for pass in game_passes {
        let pass_resource = Resource::new(resource_types::GAME_PASS, &pass.target_id.to_string())
            .add_ref_input("startPlaceId", &experience_start_place_id_ref)
            .add_value_input("name", &pass.name)?
            .add_value_input("description", &Some(pass.description))?
            .add_value_input("price", &pass.price_in_robux)?
            // TODO: should we get legit values? e.g. pass the URL
            .add_value_input("iconFilePath", &"fake_file")?
            .set_outputs(GamePassOutputs {
                asset_id: pass.target_id,
                initial_icon_asset_id: pass.icon_image_asset_id,
            })?
            .clone();
        resources.push(pass_resource.clone());
        resources.push(
            Resource::new(resource_types::GAME_PASS_ICON, &pass.target_id.to_string())
                .add_ref_input("gamePassId", &pass_resource.get_input_ref("assetId"))
                .add_ref_input(
                    "initialAssetId",
                    &pass_resource.get_input_ref("initialIconAssetId"),
                )
                // TODO: should we get legit values? e.g. pass the URL and its real hash?
                .add_value_input("filePath", &"fake_file")?
                .add_value_input("fileHash", &"fake_hash")?
                .set_outputs(GamePassIconOutputs {
                    asset_id: pass.icon_image_asset_id,
                })?
                .clone(),
        )
    }

    let badges = roblox_api.get_all_badges(experience_id)?;
    for badge in badges {
        let badge_resource = Resource::new(resource_types::BADGE, &badge.id.to_string())
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input("name", &badge.name)?
            .add_value_input("description", &badge.description)?
            .add_value_input("enabled", &badge.enabled)?
            // TODO: should we get legit values? e.g. pass the URL
            .add_value_input("iconFilePath", &"fake_file")?
            .set_outputs(BadgeOutputs {
                asset_id: badge.id,
                initial_icon_asset_id: badge.icon_image_id,
            })?
            .clone();
        resources.push(badge_resource.clone());
        resources.push(
            Resource::new(resource_types::BADGE_ICON, &badge.id.to_string())
                .add_ref_input("badgeId", &badge_resource.get_input_ref("assetId"))
                .add_ref_input(
                    "initialAssetId",
                    &badge_resource.get_input_ref("initialIconAssetId"),
                )
                // TODO: should we get legit values? e.g. pass the URL and its real hash?
                .add_value_input("filePath", &"fake_file")?
                .add_value_input("fileHash", &"fake_hash")?
                .set_outputs(BadgeIconOutputs {
                    asset_id: badge.icon_image_id,
                })?
                .clone(),
        )
    }

    let assets = roblox_api.get_all_asset_aliases(experience_id)?;
    for asset in assets {
        let resource_type = match asset.asset.type_id {
            1 => Some(resource_types::IMAGE_ASSET),
            3 => Some(resource_types::AUDIO_ASSET),
            _ => None,
        };

        if let Some(resource_type) = resource_type {
            let mut asset_resource = Resource::new(resource_type, &asset.name)
                // TODO: should we get legit values? e.g. pass the URL and its real hash?
                .add_value_input("filePath", &"fake_file")?
                .add_value_input("fileHash", &"fake_hash")?
                .add_value_input("groupId", &group_id)?
                .clone();
            match resource_type {
                resource_types::IMAGE_ASSET => {
                    asset_resource.set_outputs(ImageAssetOutputs {
                        decal_asset_id: None,
                        asset_id: asset.target_id,
                    })?;
                }
                resource_types::AUDIO_ASSET => {
                    asset_resource.set_outputs(AudioAssetOutputs {
                        asset_id: asset.target_id,
                    })?;
                }
                _ => unreachable!(),
            }
            resources.push(asset_resource.clone());
            resources.push(
                Resource::new(resource_types::ASSET_ALIAS, &asset.name)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_ref_input("assetId", &asset_resource.get_input_ref("assetId"))
                    .add_value_input("name", &asset.name)?
                    .set_outputs(AssetAliasOutputs { name: asset.name })?
                    .clone(),
            );
        }
    }

    Ok(ResourceGraph::new(&resources))
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
    state: &ResourceStateV2,
) -> Result<(), String> {
    let data = serde_yaml::to_vec(&ResourceState::Versioned(VersionedResourceState::V2(
        state.to_owned(),
    )))
    .map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    match state_config {
        StateConfig::Local => save_state_to_file(project_path, &data),
        StateConfig::Remote(config) => save_state_to_remote(config, &data).await,
    }
}
