use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::roblox_resource_manager::RobloxResource;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV5 {
    pub environments: BTreeMap<String, Vec<RobloxResource>>,
}
