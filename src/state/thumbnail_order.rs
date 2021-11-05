use serde::{Deserialize, Serialize};

use super::AssetId;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOrderResource {
    pub asset_ids: Vec<AssetId>,
}

impl ThumbnailOrderResource {
    pub fn get_id(&self) -> String {
        "thumbnailOrder-only".to_owned()
    }

    pub fn get_asset_id(&self) -> Option<AssetId> {
        // not sure what's best here
        Some(self.asset_ids.iter().sum())
    }

    pub fn get_hash(&self) -> Option<String> {
        Some(super::get_hash(
            self.asset_ids
                .iter()
                .fold("".to_owned(), |acc, id| format!("{},{}", acc, id))
                .as_bytes(),
        ))
    }

    pub fn keep(&self) -> ThumbnailOrderResource {
        self.clone()
    }

    pub fn update(&self) -> ThumbnailOrderResource {
        self.clone()
    }
}
