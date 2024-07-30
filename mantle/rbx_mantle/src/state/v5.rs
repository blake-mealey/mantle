use std::{
    collections::{BTreeMap, HashMap},
    iter::once,
};

use serde::{Deserialize, Serialize};

use crate::resource_graph::Resource;

use super::{super::roblox_resource_manager::RobloxResource, v6::ResourceStateV6, RobloxInputs};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV5 {
    pub environments: BTreeMap<String, Vec<RobloxResource>>,
}

impl From<ResourceStateV5> for ResourceStateV6 {
    fn from(state: ResourceStateV5) -> Self {
        let mut environments = BTreeMap::new();

        for (environment_name, resources) in state.environments {
            let mut environment: Vec<RobloxResource> = Vec::new();

            let resource_by_id = &resources
                .iter()
                .map(|resource| (resource.get_id(), resource.to_owned()))
                .collect::<HashMap<_, _>>();

            for resource in resources {
                match resource.get_inputs() {
                    // Add the experience singleton as a dependency to all PlaceFile resources
                    RobloxInputs::PlaceFile(_) => {
                        let dependencies = resource
                            .get_dependencies()
                            .iter()
                            .map(|id| resource_by_id.get(id).unwrap())
                            .chain(once(resource_by_id.get("experience_singleton").unwrap()))
                            .collect::<Vec<_>>();
                        environment.push(RobloxResource::existing(
                            &resource.get_id(),
                            resource.get_inputs(),
                            resource.get_outputs().unwrap(),
                            &dependencies,
                        ))
                    }
                    RobloxInputs::ProductIcon(_) => {}
                    _ => {
                        environment.push(resource.clone());
                    }
                }
            }

            environments.insert(environment_name.to_owned(), environment);
        }

        ResourceStateV6 { environments }
    }
}
