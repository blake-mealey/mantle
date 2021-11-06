use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    commands::deploy::{Config, DeploymentConfig},
    roblox_api::RobloxApi,
    roblox_auth::RobloxAuth,
};

use self::{
    experience::ExperienceResource,
    image::{ImageResource, ImageResourceType},
    place::PlaceResource,
};

mod experience;
mod image;
mod place;
mod thumbnail_order;

pub type AssetId = u64;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "version", content = "state")]
pub enum StateRoot {
    #[serde(rename = "1")]
    V1(StateV1),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StateV1 {
    pub experience: ExperienceResource,

    #[serde(default = "Vec::new")]
    pub places: Vec<PlaceResource>,

    #[serde(default = "Vec::new")]
    pub images: Vec<ImageResource>,
}

impl StateV1 {
    pub fn new(resources: &Vec<Resource>) -> Result<Self, String> {
        let experience = resources
            .iter()
            .find_map(|r| match r {
                Resource::Experience(e) => Some(e.clone()),
                _ => None,
            })
            .ok_or("Cannot construct state with no experience resource.".to_owned())?;

        let places = resources
            .iter()
            .filter_map(|r| match r {
                Resource::Place(p) => Some(p.clone()),
                _ => None,
            })
            .collect();

        let images = resources
            .iter()
            .filter_map(|r| match r {
                Resource::Image(p) => Some(p.clone()),
                _ => None,
            })
            .collect();

        Ok(Self {
            experience,
            places,
            images,
        })
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Resource {
    Experience(ExperienceResource),
    Place(PlaceResource),
    Image(ImageResource),
}

fn get_state_file_path(project_path: &Path) -> PathBuf {
    project_path.join(".rocat-state.yml")
}

pub fn get_hash(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    format!("{:x}", digest)
}

pub fn get_file_hash(file_path: &Path) -> Result<String, String> {
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
    deployment_config: &DeploymentConfig,
) -> Result<StateRoot, String> {
    let state_file_path = get_state_file_path(project_path);

    if !state_file_path.exists() {
        let experience = ExperienceResource {
            asset_id: deployment_config.experience_id.clone(),
        };

        let places = deployment_config
            .place_ids
            .iter()
            .map(|(name, id)| PlaceResource {
                name: name.clone(),
                asset_id: id.clone(),
                file_hash: None,
                file_path: None,
                version: None,
            })
            .collect();

        return Ok(StateRoot::V1(StateV1 {
            experience,
            places,
            images: Vec::new(),
        }));
    }

    let data = fs::read_to_string(&state_file_path).map_err(|e| {
        format!(
            "Unable to read state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    serde_yaml::from_str::<StateRoot>(&data).map_err(|e| {
        format!(
            "Unable to parse state file {}\n\t{}",
            state_file_path.display(),
            e
        )
    })
}

pub fn get_desired_state(
    project_path: &Path,
    config: &Config,
    deployment_config: &DeploymentConfig,
) -> Result<StateRoot, String> {
    let experience = ExperienceResource {
        asset_id: deployment_config.experience_id.clone(),
    };

    let mut places: Vec<PlaceResource> = Vec::new();
    for (name, id) in &deployment_config.place_ids {
        let file_name = config
            .place_files
            .get(name)
            .ok_or(format!("No place file configured for place {}", name))?;
        places.push(PlaceResource {
            name: name.clone(),
            asset_id: id.clone(),
            file_hash: Some(get_file_hash(project_path.join(file_name).as_path())?),
            file_path: Some(file_name.to_owned()),
            version: None,
        });
    }

    let mut images: Vec<ImageResource> = Vec::new();
    if let Some(experience_template) = &config.templates.experience {
        if let Some(icon) = &experience_template.icon {
            images.push(ImageResource {
                image_type: ImageResourceType::GameIcon,
                file_path: icon.clone(),
                file_hash: get_file_hash(project_path.join(icon).as_path())?,
                asset_id: None,
            });
        }

        if let Some(thumbnails) = &experience_template.thumbnails {
            for thumbnail in thumbnails {
                images.push(ImageResource {
                    image_type: ImageResourceType::GameThumbnail,
                    file_path: thumbnail.clone(),
                    file_hash: get_file_hash(project_path.join(thumbnail).as_path())?,
                    asset_id: None,
                });
            }
        }
    }

    Ok(StateRoot::V1(StateV1 {
        experience,
        places,
        images,
    }))
}

enum ResourceOp {
    Create,
    Keep,
    Update,
    Delete,
}

fn get_id(resource: &Resource) -> String {
    match resource {
        Resource::Experience(experience) => experience.get_id(),
        Resource::Place(place) => place.get_id(),
        Resource::Image(image) => image.get_id(),
    }
}

fn get_asset_id(resource: &Resource) -> Option<AssetId> {
    match resource {
        Resource::Experience(experience) => experience.get_asset_id(),
        Resource::Place(place) => place.get_asset_id(),
        Resource::Image(image) => image.get_asset_id(),
    }
}

fn get_resource_hash(resource: &Resource) -> Option<String> {
    match resource {
        // TODO: hash configuration
        Resource::Experience(experience) => experience.get_hash(),
        Resource::Place(place) => place.get_hash(),
        Resource::Image(image) => image.get_hash(),
    }
}

fn find_resource(resources: &Vec<Resource>, resource: &Resource) -> Option<Resource> {
    let resource_id = get_id(resource);
    resources.iter().find_map(|r| match get_id(r) {
        id if id == resource_id => Some(r.clone()),
        _ => None,
    })
}

fn get_op_for_desired_resource(
    previous_resources: &Vec<Resource>,
    desired_resource: &Resource,
) -> (ResourceOp, Option<Resource>) {
    let previous_resource = find_resource(previous_resources, desired_resource);
    let op = match &previous_resource {
        None => ResourceOp::Create,
        Some(r) if matches!(get_asset_id(&r), None) => ResourceOp::Create,
        Some(r) if get_resource_hash(&r) != get_resource_hash(desired_resource) => {
            ResourceOp::Update
        }
        _ => ResourceOp::Keep,
    };
    (op, previous_resource)
}

fn get_op_for_previous_resource(
    next_resources: &Vec<Resource>,
    previous_resource: &Resource,
) -> Option<ResourceOp> {
    let next_resource = find_resource(next_resources, previous_resource);
    match next_resource {
        None => Some(ResourceOp::Delete),
        _ => None,
    }
}

fn execute_op(
    project_path: &Path,
    roblox_api: &mut RobloxApi,
    deployment_config: &DeploymentConfig,
    desired_state: &StateV1,
    previous_resource: &Option<Resource>,
    resource: &Resource,
    op: ResourceOp,
) -> Result<Option<Resource>, String> {
    match resource {
        Resource::Experience(experience) => match op {
            ResourceOp::Keep => Ok(Some(Resource::Experience(experience.keep()))),
            // TODO: configure experience
            ResourceOp::Update => Ok(Some(Resource::Experience(experience.update()))),
            _ => panic!("Not implemented for experience"),
        },
        Resource::Place(place) => match op {
            ResourceOp::Keep => {
                if let Some(Resource::Place(previous_place)) = previous_resource {
                    return Ok(Some(Resource::Place(place.keep(&previous_place))));
                }
                unreachable!()
            }
            ResourceOp::Update => Ok(Some(Resource::Place(place.update(
                project_path,
                roblox_api,
                deployment_config,
                desired_state,
            )?))),
            _ => panic!("Not implemented for place"),
        },
        Resource::Image(image) => match op {
            ResourceOp::Keep => {
                if let Some(Resource::Image(previous_image)) = previous_resource {
                    return Ok(Some(Resource::Image(image.keep(&previous_image))));
                }
                unreachable!()
            }
            ResourceOp::Create => Ok(Some(Resource::Image(image.create(
                project_path,
                roblox_api,
                desired_state,
            )?))),
            ResourceOp::Update => Ok(Some(Resource::Image(image.update(
                project_path,
                roblox_api,
                desired_state,
            )?))),
            ResourceOp::Delete => {
                image.delete(roblox_api, desired_state)?;
                Ok(None)
            }
        },
    }
}

fn get_resources(state: &StateV1) -> Vec<Resource> {
    let mut resources: Vec<Resource> = Vec::new();

    resources.push(Resource::Experience(state.experience.clone()));
    resources.extend(state.places.iter().map(|r| Resource::Place(r.clone())));
    resources.extend(state.images.iter().map(|r| Resource::Image(r.clone())));

    resources
}

pub fn get_next_state(
    project_path: &Path,
    previous_state: &StateRoot,
    desired_state: &StateRoot,
    deployment_config: &DeploymentConfig,
) -> Result<StateRoot, String> {
    let StateRoot::V1(previous_state) = previous_state;
    let StateRoot::V1(desired_state) = desired_state;

    let previous_resources = get_resources(&previous_state);
    let desired_resources = get_resources(&desired_state);

    let mut roblox_api = RobloxApi::new(RobloxAuth::new());

    let mut next_resources: Vec<Resource> = Vec::new();

    for desired_resource in &desired_resources {
        let (op, previous_resource) =
            get_op_for_desired_resource(&previous_resources, desired_resource);

        if let Some(next_resource) = execute_op(
            project_path,
            &mut roblox_api,
            deployment_config,
            desired_state,
            &previous_resource,
            desired_resource,
            op,
        )? {
            next_resources.push(next_resource);
        }
    }

    for previous_resource in &previous_resources {
        let op = get_op_for_previous_resource(&desired_resources, previous_resource);

        if let Some(op) = op {
            execute_op(
                project_path,
                &mut roblox_api,
                deployment_config,
                desired_state,
                &None,
                previous_resource,
                op,
            )?;
        }
    }

    return Ok(StateRoot::V1(StateV1::new(&next_resources)?));
}

pub fn save_state(project_path: &Path, state: &StateRoot) -> Result<(), String> {
    let state_file_path = get_state_file_path(project_path);

    let data =
        serde_yaml::to_vec(state).map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

    fs::write(&state_file_path, data).map_err(|e| {
        format!(
            "Unable to write state file: {}\n\t{}",
            state_file_path.display(),
            e
        )
    })?;

    Ok(())
}
