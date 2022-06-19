use std::collections::HashMap;

use rbx_resource_manager::RobloxResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV4 {
    pub environments: HashMap<String, Vec<RobloxResource>>,
}
