use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupRolesResponse {
    pub group_id: AssetId,
    pub roles: Vec<ListGroupRolesResponseItem>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupRolesResponseItem {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub rank: u32,
    pub member_count: u64,
}
