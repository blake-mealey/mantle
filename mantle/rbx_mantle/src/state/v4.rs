use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{resource_graph::Resource, roblox_resource_manager::RobloxInputs};

use super::{super::roblox_resource_manager::RobloxResource, v5::ResourceStateV5};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV4 {
    pub environments: HashMap<String, Vec<RobloxResource>>,
}

impl From<ResourceStateV4> for ResourceStateV5 {
    fn from(state: ResourceStateV4) -> Self {
        let mut environments = HashMap::new();

        for (environment_name, resources) in state.environments {
            let mut environment: Vec<RobloxResource> = Vec::new();

            let resource_by_id = &resources
                .iter()
                .map(|resource| (resource.get_id(), resource.to_owned()))
                .collect::<HashMap<_, _>>();

            for resource in resources {
                match resource.get_inputs() {
                    RobloxInputs::Product(_) => {
                        let dependencies = resource.get_dependencies();
                        let product_icon_id =
                            dependencies.iter().find(|id| id.starts_with("productIcon"));
                        if let Some(product_icon_id) = product_icon_id {
                            if let Some(product_icon_resource) = resource_by_id.get(product_icon_id)
                            {
                                environment.push(RobloxResource::existing(
                                    &product_icon_resource.get_id(),
                                    product_icon_resource.get_inputs(),
                                    product_icon_resource.get_outputs().unwrap(),
                                    &[&resource],
                                ));
                            }
                        }
                        environment.push(RobloxResource::existing(
                            &resource.get_id(),
                            resource.get_inputs(),
                            resource.get_outputs().unwrap(),
                            &[resource_by_id.get("experience_singleton").unwrap()],
                        ));
                    }
                    RobloxInputs::ProductIcon(_) => {}
                    _ => {
                        environment.push(resource.clone());
                    }
                }
            }

            environments.insert(environment_name.to_owned(), environment);
        }

        ResourceStateV5 { environments }
    }
}
