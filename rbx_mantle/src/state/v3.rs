use std::collections::HashMap;

use rbx_api::models::AssetId;
use rbx_mantle_resource_graph::{Resource, ResourceId};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::{
    roblox_resource_manager::{
        PassInputs, PassOutputs, RobloxInputs, RobloxOutputs, RobloxResource,
    },
    state::v4::ResourceStateV4,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV3 {
    pub environments: HashMap<String, Vec<Value>>,
}
impl From<ResourceStateV3> for ResourceStateV4 {
    fn from(state: ResourceStateV3) -> Self {
        let mut environments = HashMap::new();

        for (environment_name, resources) in state.environments.iter() {
            let mut environment: Vec<RobloxResource> = Vec::new();
            let mut resource_by_id: HashMap<String, RobloxResource> = HashMap::new();

            let mut pass_icons: HashMap<String, RobloxResourceV3> = HashMap::new();
            let mut passes: HashMap<String, RobloxResourceV3> = HashMap::new();

            for resource in resources {
                if let Ok(resource_v4) = serde_yaml::from_value::<RobloxResource>(resource.clone())
                {
                    environment.push(resource_v4.clone());
                    resource_by_id.insert(resource_v4.get_id(), resource_v4);
                } else if let Ok(resource_v3) =
                    serde_yaml::from_value::<RobloxResourceV3>(resource.clone())
                {
                    match resource_v3.clone().inputs {
                        RobloxInputsV3::Pass(inputs) => {
                            passes.insert(inputs.icon_file_path, resource_v3);
                        }
                        RobloxInputsV3::PassIcon(inputs) => {
                            pass_icons.insert(inputs.file_path, resource_v3);
                        }
                    }
                }
            }

            for (file_path, pass) in passes {
                match (pass.inputs, pass.outputs) {
                    (RobloxInputsV3::Pass(inputs), RobloxOutputsV3::Pass(outputs)) => {
                        if let Some(pass_icon) = pass_icons.get(&file_path) {
                            match (pass_icon.clone().inputs, pass_icon.clone().outputs) {
                                (
                                    RobloxInputsV3::PassIcon(icon_inputs),
                                    RobloxOutputsV3::PassIcon(icon_outputs),
                                ) => {
                                    environment.push(RobloxResource::existing(
                                        &pass.id,
                                        RobloxInputs::Pass(PassInputs {
                                            name: inputs.name,
                                            description: inputs.description,
                                            icon_file_path: inputs.icon_file_path,
                                            icon_file_hash: icon_inputs.file_hash,
                                            price: inputs.price,
                                        }),
                                        RobloxOutputs::Pass(PassOutputs {
                                            asset_id: outputs.asset_id,
                                            icon_asset_id: icon_outputs.asset_id,
                                        }),
                                        &[resource_by_id.get("experience_singleton").unwrap()],
                                    ));
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            environment.push(RobloxResource::existing(
                                &pass.id,
                                RobloxInputs::Pass(PassInputs {
                                    name: inputs.name,
                                    description: inputs.description,
                                    icon_file_path: inputs.icon_file_path,
                                    icon_file_hash: "unknown".to_owned(),
                                    price: inputs.price,
                                }),
                                RobloxOutputs::Pass(PassOutputs {
                                    asset_id: outputs.asset_id,
                                    icon_asset_id: outputs.initial_icon_asset_id,
                                }),
                                &[resource_by_id.get("experience_singleton").unwrap()],
                            ))
                        }
                    }
                    _ => unreachable!(),
                }
            }

            environments.insert(environment_name.to_owned(), environment);
        }

        ResourceStateV4 { environments }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobloxResourceV3 {
    id: ResourceId,
    inputs: RobloxInputsV3,
    outputs: RobloxOutputsV3,
    dependencies: Vec<ResourceId>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum RobloxInputsV3 {
    Pass(PassInputsV3),
    PassIcon(FileInputsV3),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileInputsV3 {
    pub file_path: String,
    pub file_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassInputsV3 {
    pub name: String,
    pub description: String,
    pub price: Option<u32>,
    pub icon_file_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RobloxOutputsV3 {
    Pass(AssetWithInitialIconOutputsV3),
    PassIcon(AssetOutputsV3),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetWithInitialIconOutputsV3 {
    pub asset_id: AssetId,
    pub initial_icon_asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetOutputsV3 {
    pub asset_id: AssetId,
}
