use serde::{Deserialize, Serialize};

use crate::models::AssetId;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetRequest {
    pub asset_type: String,
    pub display_name: String,
    pub description: String,
    pub creation_context: CreateAssetContext,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetContext {
    pub creator: Creator,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Creator {
    UserId(String),
    GroupId(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResult<TResponse> {
    pub path: String,
    pub done: bool,
    pub response: Option<TResponse>,
    pub error: Option<OperationError>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationError {
    pub error: String,
    pub message: String,
    pub details: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_type: String,
    pub asset_id: String,
}

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
