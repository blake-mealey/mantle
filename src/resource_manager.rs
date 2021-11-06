use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    commands::deploy::DeploymentConfig,
    resources::ResourceManagerBackend,
    roblox_api::{
        self, ExperienceConfigurationModel, PlaceConfigurationModel, RobloxApi, UploadImageResult,
        UploadPlaceResult,
    },
    roblox_auth::RobloxAuth,
};

pub type AssetId = u64;

pub mod resource_types {
    pub const EXPERIENCE: &str = "experience";
    pub const EXPERIENCE_ACTIVATION: &str = "experience_activation";
    pub const EXPERIENCE_ICON: &str = "experience_icon";
    pub const EXPERIENCE_THUMBNAIL: &str = "experience_thumbnail";
    pub const EXPERIENCE_THUMBNAIL_ORDER: &str = "experience_thumbnail_order";
    pub const PLACE_FILE: &str = "place_file";
    pub const PLACE_CONFIGURATION: &str = "place_configuration";
}

pub const SINGLETON_RESOURCE_ID: &str = "singleton";

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceInputs {
    configuration: ExperienceConfigurationModel,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceActivationInputs {
    experience_id: AssetId,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceIconInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceIconOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailOrderInputs {
    experience_id: AssetId,
    asset_ids: Vec<AssetId>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceFileInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceFileOutputs {
    #[serde(default)]
    version: u32,
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceConfigurationInputs {
    experience_id: AssetId,
    asset_id: AssetId,
    configuration: PlaceConfigurationModel,
}

pub struct RobloxResourceManager {
    roblox_api: RobloxApi,
    project_path: PathBuf,
    deployment_config: DeploymentConfig,
}

impl RobloxResourceManager {
    pub fn new(project_path: &Path, deployment_config: &DeploymentConfig) -> Self {
        Self {
            roblox_api: RobloxApi::new(RobloxAuth::new()),
            project_path: project_path.to_path_buf(),
            deployment_config: deployment_config.clone(),
        }
    }
}

impl ResourceManagerBackend for RobloxResourceManager {
    fn create(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        // println!(
        //     "CREATE: {} {}",
        //     resource_type,
        //     serde_yaml::to_string(&resource_inputs).map_err(|_| "".to_owned())?
        // );
        match resource_type {
            resource_types::EXPERIENCE_ACTIVATION => {
                let inputs = serde_yaml::from_value::<ExperienceActivationInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .set_experience_active(inputs.experience_id, inputs.is_active)?;

                Ok(None)
            }
            resource_types::EXPERIENCE_ICON => {
                let inputs = serde_yaml::from_value::<ExperienceIconInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let UploadImageResult { asset_id } = self.roblox_api.upload_icon(
                    inputs.experience_id,
                    self.project_path.join(inputs.file_path).as_path(),
                )?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceIconOutputs { asset_id })
                        .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::EXPERIENCE_THUMBNAIL => {
                let inputs = serde_yaml::from_value::<ExperienceThumbnailInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let UploadImageResult { asset_id } = self.roblox_api.upload_thumbnail(
                    inputs.experience_id,
                    self.project_path.join(inputs.file_path).as_path(),
                )?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceThumbnailOutputs { asset_id })
                        .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
                let inputs =
                    serde_yaml::from_value::<ExperienceThumbnailOrderInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .set_experience_thumbnail_order(inputs.experience_id, &inputs.asset_ids)?;

                Ok(None)
            }
            _ => panic!(
                "Create not implemented for resource type: {}",
                resource_type
            ),
        }
    }

    fn update(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        // println!("UPDATE: {} {:?}", resource_type, resource_inputs);
        match resource_type {
            resource_types::EXPERIENCE => {
                let inputs = serde_yaml::from_value::<ExperienceInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs = serde_yaml::from_value::<ExperienceOutputs>(resource_outputs)
                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                self.roblox_api
                    .configure_experience(outputs.asset_id, &inputs.configuration)?;

                Ok(None)
            }
            resource_types::EXPERIENCE_ACTIVATION => {
                self.create(resource_type, resource_inputs.clone())
            }
            resource_types::EXPERIENCE_ICON => self.create(resource_type, resource_inputs.clone()),
            resource_types::EXPERIENCE_THUMBNAIL => {
                self.delete(
                    resource_type,
                    resource_inputs.clone(),
                    resource_outputs.clone(),
                )?;
                self.create(resource_type, resource_inputs.clone())
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
                self.create(resource_type, resource_inputs.clone())
            }
            resource_types::PLACE_FILE => {
                let inputs = serde_yaml::from_value::<PlaceFileInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs = serde_yaml::from_value::<PlaceFileOutputs>(resource_outputs)
                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                let UploadPlaceResult { place_version } = self.roblox_api.upload_place(
                    self.project_path.join(inputs.file_path).as_path(),
                    inputs.experience_id,
                    outputs.asset_id,
                    self.deployment_config.deploy_mode,
                )?;

                Ok(Some(
                    serde_yaml::to_value(PlaceFileOutputs {
                        version: place_version,
                        asset_id: outputs.asset_id,
                    })
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::PLACE_CONFIGURATION => {
                let inputs = serde_yaml::from_value::<PlaceConfigurationInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .configure_place(inputs.asset_id, &inputs.configuration)?;

                Ok(None)
            }
            _ => panic!(
                "Update not implemented for resource type: {}",
                resource_type
            ),
        }
    }

    fn delete(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<(), String> {
        // println!("DELETE: {} {:?}", resource_type, resource_outputs);
        match resource_type {
            resource_types::EXPERIENCE_ICON => {
                // TODO: figure out which endpoint to use to delete an icon
                Ok(())
            }
            resource_types::EXPERIENCE_THUMBNAIL => {
                let inputs = serde_yaml::from_value::<ExperienceThumbnailInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs =
                    serde_yaml::from_value::<ExperienceThumbnailOutputs>(resource_outputs)
                        .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                self.roblox_api
                    .delete_experience_thumbnail(inputs.experience_id, outputs.asset_id)
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => Ok(()),
            _ => panic!(
                "Delete not implemented for resource type: {}",
                resource_type
            ),
        }
    }
}
