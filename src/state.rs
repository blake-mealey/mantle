use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use yansi::Paint;

use crate::{
    config::{Config, DeploymentConfig, PlayabilityConfig, RemoteStateConfig, StateConfig},
    logger,
    resource_manager::{resource_types, SINGLETON_RESOURCE_ID},
    resources::{InputRef, Resource, ResourceGraph},
    roblox_api::{ExperienceConfigurationModel, PlaceConfigurationModel},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceState {
    pub deployments: HashMap<String, Vec<Resource>>,
}

fn get_state_file_path(project_path: &Path) -> PathBuf {
    project_path.join(".rocat-state.yml")
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
            key: format!("{}.rocat-state.yml", config.key),
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
    deployment_config: &DeploymentConfig,
) -> Result<ResourceState, String> {
    let state = match config.state.clone() {
        StateConfig::Local => get_state_from_file(project_path)?,
        StateConfig::Remote(config) => get_state_from_remote(&config).await?,
    };

    let mut state = match state {
        Some(state) => state,
        None => ResourceState {
            deployments: HashMap::new(),
        },
    };

    if state.deployments.get(&deployment_config.name).is_none() {
        logger::log(format!(
            "No previous state for deployment {}",
            Paint::cyan(deployment_config.name.clone())
        ));
        state
            .deployments
            .insert(deployment_config.name.clone(), Vec::new());
    }

    Ok(state)
}

pub fn get_desired_graph(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<ResourceGraph, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_value_input("assetId", &deployment_config.experience_id.clone())?
        .clone();
    let experience_asset_id_ref = experience.get_input_ref("assetId");
    let experience_start_place_id_ref = experience.get_input_ref("startPlaceId");
    resources.push(experience);

    if let Some(experience_configuration) = &config.templates.experience {
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
                        Some(PlayabilityConfig::Private)
                    ),
                )?
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .clone(),
        );

        if let Some(file_path) = &experience_configuration.icon {
            resources.push(
                Resource::new(resource_types::EXPERIENCE_ICON, file_path)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
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

    if !config.templates.places.keys().any(|n| n == "start") {
        return Err("No start place defined".to_owned());
    }

    for (name, template) in config.templates.places.iter() {
        let place_id = deployment_config.place_ids.get(name);

        let place = Resource::new(resource_types::PLACE, name)
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_ref_input("startPlaceId", &experience_start_place_id_ref)
            .add_value_input("assetId", &place_id)?
            .add_value_input("isStart", &(name == "start"))?
            .clone();
        let place_asset_id_ref = place.get_input_ref("assetId");
        resources.push(place);

        resources.push(
            Resource::new(resource_types::PLACE_FILE, name)
                .add_ref_input("assetId", &place_asset_id_ref)
                .add_value_input("filePath", &template.file)?
                .add_value_input(
                    "fileHash",
                    &get_file_hash(project_path.join(&template.file).as_path())?,
                )?
                .clone(),
        );

        resources.push(
            Resource::new(resource_types::PLACE_CONFIGURATION, name)
                .add_ref_input("assetId", &place_asset_id_ref)
                .add_value_input::<PlaceConfigurationModel>(
                    "configuration",
                    &template.clone().into(),
                )?
                .clone(),
        );
    }

    if let Some(developer_products) = &config.templates.products {
        for (name, developer_product) in developer_products {
            let mut product_resource = Resource::new(resource_types::DEVELOPER_PRODUCT, name)
                .add_ref_input("experienceId", &experience_asset_id_ref)
                .add_value_input("name", &developer_product.name)?
                .add_value_input("price", &developer_product.price)?
                .add_value_input(
                    "description",
                    developer_product
                        .description
                        .as_ref()
                        .unwrap_or(&"".to_owned()),
                )?
                .clone();
            if let Some(icon_path) = &developer_product.icon {
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

    if let Some(passes) = &config.templates.passes {
        for (name, pass_config) in passes {
            let pass_resource = Resource::new(resource_types::GAME_PASS, name)
                .add_ref_input("startPlaceId", &experience_start_place_id_ref)
                .add_value_input("name", &pass_config.name)?
                .add_value_input("description", &pass_config.description)?
                .add_value_input("price", &pass_config.price)?
                .add_value_input("iconFilePath", &pass_config.icon)?
                .clone();
            resources.push(pass_resource.clone());
            resources.push(
                Resource::new(resource_types::GAME_PASS_ICON, name)
                    .add_ref_input("gamePassId", &pass_resource.get_input_ref("assetId"))
                    .add_ref_input(
                        "initialAssetId",
                        &pass_resource.get_input_ref("initialIconAssetId"),
                    )
                    .add_value_input("filePath", &pass_config.icon)?
                    .add_value_input(
                        "fileHash",
                        &get_file_hash(project_path.join(&pass_config.icon).as_path())?,
                    )?
                    .clone(),
            )
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
            key: format!("{}.rocat-state.yml", config.key),
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
    state: &ResourceState,
) -> Result<(), String> {
    let data =
        serde_yaml::to_vec(&state).map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    match state_config {
        StateConfig::Local => save_state_to_file(project_path, &data),
        StateConfig::Remote(config) => save_state_to_remote(config, &data).await,
    }
}
