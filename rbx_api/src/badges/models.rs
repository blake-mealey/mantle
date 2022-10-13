use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateBadgeResponse {
    pub id: AssetId,
    pub icon_image_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBadgesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListBadgeResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListBadgeResponse {
    pub id: AssetId,
    pub name: String,
    pub description: String,
    pub icon_image_id: AssetId,
    pub enabled: bool,
}
