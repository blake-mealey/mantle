use std::{fmt::Display, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    commands::deploy::DeploymentConfig,
    roblox_api::{RobloxApi, UploadImageResult},
};

use super::{AssetId, StateV1};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ImageResourceType {
    GameIcon,
    GameThumbnail,
}

impl Display for ImageResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageResourceType::GameIcon => "gameIcon",
                ImageResourceType::GameThumbnail => "gameThumbnail",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource {
    pub image_type: ImageResourceType,
    pub file_path: String,
    pub file_hash: String,
    pub asset_id: Option<AssetId>,
}

impl ImageResource {
    pub fn get_id(&self) -> String {
        format!("{}-{}", self.image_type, self.file_path)
    }

    pub fn get_asset_id(&self) -> Option<AssetId> {
        self.asset_id.clone()
    }

    pub fn get_hash(&self) -> Option<String> {
        Some(self.file_hash.clone())
    }

    pub fn keep(&self, previous_image: &Self) -> ImageResource {
        ImageResource {
            image_type: self.image_type.clone(),
            file_path: self.file_path.clone(),
            file_hash: self.file_hash.clone(),
            asset_id: previous_image.asset_id.clone(),
        }
    }

    pub fn create(
        &self,
        project_path: &Path,
        roblox_api: &mut RobloxApi,
        desired_state: &StateV1,
    ) -> Result<ImageResource, String> {
        let file_path = project_path.join(&self.file_path);
        let UploadImageResult { asset_id } = match self.image_type {
            ImageResourceType::GameIcon => {
                roblox_api.upload_icon(desired_state.experience.asset_id, file_path.as_path())?
            }
            ImageResourceType::GameThumbnail => roblox_api
                .upload_thumbnail(desired_state.experience.asset_id, file_path.as_path())?,
        };
        Ok(ImageResource {
            image_type: self.image_type.clone(),
            file_path: self.file_path.clone(),
            file_hash: self.file_hash.clone(),
            asset_id: Some(asset_id),
        })
    }

    pub fn update(
        &self,
        project_path: &Path,
        roblox_api: &mut RobloxApi,
        desired_state: &StateV1,
    ) -> Result<ImageResource, String> {
        self.delete(roblox_api, desired_state)?;
        self.create(project_path, roblox_api, desired_state)
    }

    pub fn delete(
        &self,
        roblox_api: &mut RobloxApi,
        desired_state: &StateV1,
    ) -> Result<(), String> {
        match self.image_type {
            ImageResourceType::GameThumbnail => roblox_api.delete_experience_thumbnail(
                desired_state.experience.asset_id,
                self.asset_id.ok_or("No asset id".to_owned())?,
            )?,
            _ => {}
        };
        Ok(())
    }
}
