use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateGamePassResponse {
    pub game_pass_id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetGamePassResponse {
    pub target_id: AssetId,
    pub name: String,
    pub description: String,
    pub icon_image_asset_id: AssetId,
    pub price_in_robux: Option<u32>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGamePassResponse {
    pub id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGamePassesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListGamePassResponse>,
}
