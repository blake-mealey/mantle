use serde::Deserialize;

use crate::models::GroupId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupAllRolesResponse {
    pub group_id: GroupId,
    pub roles: Vec<GroupRoleResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupRoleResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub rank: u32,
    pub member_count: u64,
}
