use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    commands::deploy::DeploymentConfig,
    roblox_api::{RobloxApi, UploadPlaceResult},
};

use super::{AssetId, StateV1};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceResource {
    pub name: String,
    // TODO: make optional for creating new places
    pub asset_id: AssetId,
    pub file_path: Option<String>,
    pub file_hash: Option<String>,
    pub version: Option<u32>,
    // TODO: configuration
}

impl PlaceResource {
    pub fn get_id(&self) -> String {
        format!("place-{}", self.name.clone())
    }

    pub fn get_asset_id(&self) -> Option<AssetId> {
        Some(self.asset_id.clone())
    }

    pub fn get_hash(&self) -> Option<String> {
        self.file_hash.clone()
    }

    pub fn keep(&self, previous_place: &Self) -> PlaceResource {
        PlaceResource {
            name: self.name.clone(),
            asset_id: self.asset_id.clone(),
            file_hash: self.file_hash.clone(),
            file_path: self.file_path.clone(),
            version: previous_place.version.clone(),
        }
    }

    pub fn update(
        &self,
        project_path: &Path,
        roblox_api: &mut RobloxApi,
        deployment_config: &DeploymentConfig,
        desired_state: &StateV1,
    ) -> Result<PlaceResource, String> {
        let UploadPlaceResult { place_version } = roblox_api.upload_place(
            project_path.join(self.file_path.clone().unwrap()).as_path(),
            desired_state.experience.asset_id,
            self.asset_id,
            deployment_config.deploy_mode,
        )?;
        Ok(PlaceResource {
            name: self.name.clone(),
            asset_id: self.asset_id.clone(),
            file_hash: self.file_hash.clone(),
            file_path: self.file_path.clone(),
            version: Some(place_version),
        })
    }
}
