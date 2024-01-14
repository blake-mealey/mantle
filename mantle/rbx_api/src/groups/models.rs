use serde::Deserialize;

use crate::models::{Group, Id, Role};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupRolesResponse {
    pub group_id: Id<Group>,
    pub roles: Vec<ListGroupRolesResponseItem>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupRolesResponseItem {
    pub id: Id<Role>,
    pub name: String,
    pub description: Option<String>,
    pub rank: u32,
    pub member_count: u64,
}
