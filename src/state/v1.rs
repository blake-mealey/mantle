use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    legacy_resources::{Input, LegacyResource},
    v2::ResourceStateV2,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV1 {
    pub deployments: HashMap<String, Vec<LegacyResource>>,
}
impl From<ResourceStateV1> for ResourceStateV2 {
    fn from(state: ResourceStateV1) -> Self {
        // State format change: "deployments" -> "environments"
        let mut environments = state.deployments;
        for (_, resources) in environments.iter_mut() {
            for resource in resources {
                let r_type = resource.resource_type.as_str();

                // Resources format change: remove assetId input from experience and place resources
                // to avoid unnecessary recreation of resources
                if matches!(r_type, "experience" | "place") {
                    resource.inputs.remove("assetId");
                }

                // Resources format change: add groupId input to experience and asset resources to
                // avoid unnecessary recreation of resources
                if matches!(r_type, "experience" | "imageAsset" | "audioAsset") {
                    resource
                        .inputs
                        .insert("groupId".to_owned(), Input::Value(serde_yaml::Value::Null));
                }
            }
        }

        ResourceStateV2 { environments }
    }
}
