use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::safe_resource_manager::RobloxResource;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV3 {
    pub environments: HashMap<String, Vec<RobloxResource>>,
}
