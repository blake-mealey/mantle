use serde::Deserialize;

use crate::models::GroupId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRolesResponse {
    pub group_id: GroupId,
    pub roles: Vec<ListRoleResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListRoleResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub rank: u64,
    pub member_count: u64,
}
