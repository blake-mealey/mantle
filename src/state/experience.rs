use serde::{Deserialize, Serialize};

use super::AssetId;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceResource {
    // TODO: make optional for creating new experiences
    pub asset_id: AssetId,
    // TODO: configuration
}

impl ExperienceResource {
    pub fn get_id(&self) -> String {
        "experience-only".to_owned()
    }

    pub fn get_asset_id(&self) -> Option<AssetId> {
        Some(self.asset_id.clone())
    }

    pub fn get_hash(&self) -> Option<String> {
        None
    }

    pub fn keep(&self) -> ExperienceResource {
        self.clone()
    }

    pub fn update(&self) -> ExperienceResource {
        self.clone()
    }
}
