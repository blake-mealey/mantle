use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    config::{Config, DeploymentConfig, PlayabilityConfig},
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

pub fn get_previous_state(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<ResourceState, String> {
    let state_file_path = get_state_file_path(project_path);
    let mut state = if state_file_path.exists() {
        let data = fs::read_to_string(&state_file_path).map_err(|e| {
            format!(
                "Unable to read state file: {}\n\t{}",
                state_file_path.display(),
                e
            )
        })?;

        serde_yaml::from_str::<ResourceState>(&data).map_err(|e| {
            format!(
                "Unable to parse state file {}\n\t{}",
                state_file_path.display(),
                e
            )
        })?
    } else {
        ResourceState {
            deployments: HashMap::new(),
        }
    };

    if state.deployments.get(&deployment_config.name).is_none() {
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
                .place_files
                .get(name)
                .ok_or(format!("No place file configured for place {}", name))?;
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

        state
            .deployments
            .insert(deployment_config.name.clone(), resources);
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
            .place_files
            .get(name)
            .ok_or(format!("No place file configured for place {}", name))?;
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

pub fn save_state(project_path: &Path, state: &ResourceState) -> Result<(), String> {
    let state_file_path = get_state_file_path(project_path);

    let data =
        serde_yaml::to_vec(&state).map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    fs::write(&state_file_path, data).map_err(|e| {
        format!(
            "Unable to write state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    Ok(())
}
