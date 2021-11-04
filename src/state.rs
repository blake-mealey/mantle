use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::roblox_api::UploadImageResult;

#[derive(Deserialize, Serialize)]
#[serde(tag = "version", content = "state")]
pub enum RocatStateRoot {
    #[serde(rename = "1")]
    V1(RocatStateV1),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocatStateV1 {
    pub experience: Option<RocatExperienceStateV1>,
    #[serde(default = "HashMap::new")]
    pub places: HashMap<String, RocatPlaceStateV1>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocatExperienceStateV1 {
    pub asset_id: u64,
    pub icon: Option<RocatImageStateV1>,
    pub thumbnails: Option<Vec<RocatImageStateV1>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocatPlaceStateV1 {
    pub asset_id: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocatImageStateV1 {
    pub hash: String,
    pub asset_id: u64,
}

pub struct RocatState {
    pub path: PathBuf,
    pub state: RocatStateRoot,
}

pub fn load_state_file(state_file: &Path) -> Result<RocatStateRoot, String> {
    if !state_file.exists() {
        return Ok(RocatStateRoot::V1(RocatStateV1 {
            experience: None,
            places: HashMap::new(),
        }));
    }

    let data = fs::read_to_string(state_file).map_err(|e| {
        format!(
            "Unable to read state file: {}\n\t{}",
            state_file.display(),
            e
        )
    })?;

    serde_yaml::from_str::<RocatStateRoot>(&data).map_err(|e| {
        format!(
            "Unable to parse state file {}\n\t{}",
            state_file.display(),
            e
        )
    })
}

impl RocatState {
    pub fn load_from_file(project_path: &Path) -> Result<Self, String> {
        let state_file = project_path.join(".rocat-state.yml");
        Ok(RocatState {
            path: state_file.clone(),
            state: load_state_file(&state_file)?,
        })
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        let data = serde_yaml::to_vec(&self.state)
            .map_err(|e| format!("Unable to serialize state\n\t{}", e))?;

        fs::write(&self.path, data).map_err(|e| {
            format!(
                "Unable to write state file: {}\n\t{}",
                self.path.display(),
                e
            )
        })?;

        Ok(())
    }

    pub fn set_experience_asset_id(&mut self, asset_id: u64) {
        match &mut self.state {
            RocatStateRoot::V1(root) => Self::set_experience_asset_id_v1(root, asset_id),
        }
    }

    fn set_experience_asset_id_v1(root: &mut RocatStateV1, asset_id: u64) {
        if let Some(experience) = &mut root.experience {
            experience.asset_id = asset_id;
        } else {
            root.experience = Some(RocatExperienceStateV1 {
                asset_id,
                icon: None,
                thumbnails: None,
            });
        }
    }

    pub fn set_experience_icon(&mut self, asset_id: u64, hash: String) {
        match &mut self.state {
            RocatStateRoot::V1(root) => Self::set_experience_icon_v1(root, asset_id, hash),
        }
    }

    fn set_experience_icon_v1(root: &mut RocatStateV1, asset_id: u64, hash: String) {
        if let Some(experience) = &mut root.experience {
            experience.icon = Some(RocatImageStateV1 { asset_id, hash });
            return ();
        }
        panic!("Attempted to set the icon of an uninitialized experience.");
    }

    pub fn needs_to_upload_experience_icon(&self, hash: String) -> Result<bool, String> {
        match &self.state {
            RocatStateRoot::V1(root) => Self::needs_to_upload_experience_icon_v1(root, hash),
        }
    }

    pub fn needs_to_upload_experience_icon_v1(
        root: &RocatStateV1,
        hash: String,
    ) -> Result<bool, String> {
        if let Some(experience) = &root.experience {
            if let Some(icon) = &experience.icon {
                return Ok(icon.hash != hash);
            }
        }

        Ok(true)
    }

    pub fn needs_to_upload_experience_thumbnail(&self, hash: String) -> Result<bool, String> {
        match &self.state {
            RocatStateRoot::V1(root) => Self::needs_to_upload_experience_thumbnail_v1(root, hash),
        }
    }

    pub fn needs_to_upload_experience_thumbnail_v1(
        root: &RocatStateV1,
        hash: String,
    ) -> Result<bool, String> {
        if let Some(experience) = &root.experience {
            if let Some(thumbnails) = &experience.thumbnails {
                for thumbnail in thumbnails {
                    if thumbnail.hash == hash {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    pub fn get_experience_icon_asset_id(&self) -> u64 {
        match &self.state {
            RocatStateRoot::V1(root) => Self::get_experience_icon_asset_id_v1(root),
        }
    }

    fn get_experience_icon_asset_id_v1(root: &RocatStateV1) -> u64 {
        if let Some(experience) = &root.experience {
            if let Some(icon) = &experience.icon {
                return icon.asset_id;
            }
        }
        panic!("Attempted to get the asset_id of an uninitialized experience icon.");
    }

    pub fn get_experience_thumbnail_asset_id_from_hash(&self, hash: String) -> u64 {
        match &self.state {
            RocatStateRoot::V1(root) => {
                Self::get_experience_thumbnail_asset_id_from_hash_v1(root, hash)
            }
        }
    }

    fn get_experience_thumbnail_asset_id_from_hash_v1(root: &RocatStateV1, hash: String) -> u64 {
        if let Some(experience) = &root.experience {
            if let Some(thumbnails) = &experience.thumbnails {
                for thumbnail in thumbnails {
                    if thumbnail.hash == hash {
                        return thumbnail.asset_id;
                    }
                }
            }
        }
        panic!("Attempted to get the asset_id of a non-existent experience thumbnail.");
    }

    pub fn set_experience_thumbnails(&mut self, thumbnails: Vec<UploadImageResult>) {
        match &mut self.state {
            RocatStateRoot::V1(root) => Self::set_experience_thumbnails_v1(root, thumbnails),
        }
    }

    fn set_experience_thumbnails_v1(root: &mut RocatStateV1, thumbnails: Vec<UploadImageResult>) {
        if let Some(experience) = &mut root.experience {
            experience.thumbnails = Some(
                thumbnails
                    .into_iter()
                    .map(|t| RocatImageStateV1 {
                        asset_id: t.asset_id,
                        hash: t.hash,
                    })
                    .collect(),
            );
            return ();
        }
        panic!("Attempted to set the thumbnails of an uninitialized experience.");
    }

    pub fn get_experience_thumbnail_order(&self) -> Vec<u64> {
        match &self.state {
            RocatStateRoot::V1(root) => Self::get_experience_thumbnail_order_v1(root),
        }
    }

    fn get_experience_thumbnail_order_v1(root: &RocatStateV1) -> Vec<u64> {
        if let Some(experience) = &root.experience {
            if let Some(thumbnails) = &experience.thumbnails {
                return thumbnails.iter().map(|t| t.asset_id.clone()).collect();
            } else {
                return Vec::new();
            }
        }
        panic!("Attempted to get the thumbnail order of an uninitialized experience.");
    }
}
