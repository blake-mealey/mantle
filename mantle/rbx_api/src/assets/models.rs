use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum QuotaDuration {
    Month,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetQuota {
    pub duration: QuotaDuration,
    pub usage: u32,
    pub capacity: u32,
    pub expiration_time: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetQuotasResponse {
    pub quotas: Vec<CreateAssetQuota>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAudioAssetResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageAssetResponse {
    pub asset_id: AssetId,
    pub backing_asset_id: AssetId,
}
