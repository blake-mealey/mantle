use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAssetAliasResponse {
    pub name: String,
    pub target_id: AssetId,
    pub asset: GetAssetResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAssetResponse {
    pub type_id: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAssetAliasesResponse {
    pub aliases: Vec<GetAssetAliasResponse>,
    pub final_page: bool,
}
