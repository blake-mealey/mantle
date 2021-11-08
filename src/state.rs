use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;

use crate::{
    config::{Config, DeploymentConfig, PlayabilityConfig, RemoteStateConfig, StateConfig},
    resource_manager::{resource_types, AssetId, SINGLETON_RESOURCE_ID},
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
    println!(
        "Loading previous state from local file: {}\n",
        state_file_path.display()
    );

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
    println!("Loading previous state from remote object: {}\n", config);

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

fn get_default_resources(
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<Vec<Resource>, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let mut experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_output::<AssetId>("assetId", &deployment_config.experience_id.clone())?
        .clone();
    let experience_asset_id_ref = experience.get_input_ref("assetId");
    if config.templates.experience.is_some() {
        experience.add_value_stub_input("configuration");
    }
    resources.push(experience.clone());

    for (name, id) in deployment_config.place_ids.iter() {
        let place_file = config
            .templates
            .places
            .get(name)
            .map(|p| p.file.clone())
            .ok_or(format!("No place file configured for place: {}", name))?;
        let place_file_resource = Resource::new(resource_types::PLACE_FILE, name)
            .add_output("assetId", &id)?
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input("filePath", &place_file)?
            .add_value_stub_input("fileHash")
            .add_value_stub_input("version")
            .add_value_stub_input("deployMode")
            .clone();
        let place_file_asset_id_ref = place_file_resource.get_input_ref("assetId");
        resources.push(place_file_resource);
        if config.templates.places.contains_key(name) {
            resources.push(
                Resource::new(resource_types::PLACE_CONFIGURATION, name)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_ref_input("assetId", &place_file_asset_id_ref)
                    .add_value_stub_input("configuration")
                    .clone(),
            );
        }
    }

    Ok(resources)
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
        println!(
            "No previous state for deployment {}.",
            deployment_config.name
        );
        state.deployments.insert(
            deployment_config.name.clone(),
            get_default_resources(config, deployment_config)?,
        );
    }

    Ok(state)
}

pub fn get_desired_graph(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<ResourceGraph, String> {
    let mut resources: Vec<Resource> = Vec::new();

    let mut experience = Resource::new(resource_types::EXPERIENCE, SINGLETON_RESOURCE_ID)
        .add_output::<AssetId>("assetId", &deployment_config.experience_id.clone())?
        .clone();
    let experience_asset_id_ref = experience.get_input_ref("assetId");
    if let Some(experience_configuration) = &config.templates.experience {
        experience.add_value_input::<ExperienceConfigurationModel>(
            "configuration",
            &experience_configuration.into(),
        )?;
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
    }
    resources.push(experience.clone());

    for (name, id) in deployment_config.place_ids.iter() {
        let place_file = config
            .templates
            .places
            .get(name)
            .map(|p| p.file.clone())
            .ok_or(format!("No place file configured for place: {}", name))?;
        let place_file_resource = Resource::new(resource_types::PLACE_FILE, name)
            .add_output("assetId", &id)?
            .add_ref_input("experienceId", &experience_asset_id_ref)
            .add_value_input("filePath", &place_file)?
            .add_value_input(
                "fileHash",
                &get_file_hash(project_path.join(place_file).as_path())?,
            )?
            .add_value_input("deployMode", &deployment_config.deploy_mode)?
            .clone();
        let place_file_asset_id_ref = place_file_resource.get_input_ref("assetId");
        resources.push(place_file_resource);
        if let Some(place_configuration) = config.templates.places.get(name) {
            resources.push(
                Resource::new(resource_types::PLACE_CONFIGURATION, name)
                    .add_ref_input("experienceId", &experience_asset_id_ref)
                    .add_ref_input("assetId", &place_file_asset_id_ref)
                    .add_value_input::<PlaceConfigurationModel>(
                        "configuration",
                        &place_configuration.clone().into(),
                    )?
                    .clone(),
            );
        }
    }

    if let Some(experience_configuration) = &config.templates.experience {
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

    Ok(ResourceGraph::new(&resources))
}

pub async fn save_state_to_remote(config: &RemoteStateConfig, data: &[u8]) -> Result<(), String> {
    println!("\nSaving state to remote object: {}", config);

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

    println!("\nSaving state to local file. It is recommended you commit this file to your source control: {}", state_file_path.display());

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
