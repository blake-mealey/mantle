use std::collections::HashMap;

use rbx_mantle_resource_graph::resource::{Resource, ResourceId};
use serde::{Deserialize, Serialize};

use super::super::roblox_resource_manager::*;

use super::{
    legacy_resources::{Input, LegacyResource, LegacyResourceGraph, ResourceRef},
    v3::ResourceStateV3,
};

macro_rules! output_value {
    ($resource:expr, $input:expr) => {{
        let value = $resource
            .outputs
            .clone()
            .unwrap()
            .get($input)
            .unwrap()
            .clone();
        serde_yaml::from_value(value).unwrap()
    }};
}

macro_rules! input_value {
    ($resource:expr, $input:expr) => {{
        let value = $resource.inputs.get($input).unwrap().clone();
        match value {
            Input::Value(value) => serde_yaml::from_value(value).unwrap(),
            _ => panic!(),
        }
    }};
}

macro_rules! input_ref {
    ($resource:expr, $input:expr) => {{
        let value = $resource.inputs.get($input).unwrap().clone();
        match value {
            Input::Ref((resource_type, resource_id, _output_name)) => (resource_type, resource_id),
            _ => panic!(),
        }
    }};
}

macro_rules! optional_input_ref {
    ($resource:expr, $input:expr) => {{
        if let Some(value) = $resource.inputs.get($input) {
            match value {
                Input::Ref((resource_type, resource_id, _output_name)) => {
                    Some((resource_type.clone(), resource_id.clone()))
                }
                _ => panic!(),
            }
        } else {
            None
        }
    }};
}

macro_rules! input_ref_list {
    ($resource:expr, $input:expr) => {{
        let value = $resource.inputs.get($input).unwrap().clone();
        match value {
            Input::RefList(list) => list
                .iter()
                .map(|(resource_type, resource_id, _output_name)| {
                    (resource_type.clone(), resource_id.clone())
                })
                .collect::<Vec<_>>(),
            _ => panic!(),
        }
    }};
}

macro_rules! dependency {
    ($ref_to_resource:expr, $resource:expr, $input:expr) => {{
        $ref_to_resource
            .get(&input_ref!($resource, $input))
            .unwrap()
    }};
}

macro_rules! optional_dependency {
    ($ref_to_resource:expr, $resource:expr, $input:expr) => {{
        if let Some(resource_ref) = optional_input_ref!($resource, $input) {
            $ref_to_resource.get(&resource_ref)
        } else {
            None
        }
    }};
}

macro_rules! dependency_list {
    ($ref_to_resource:expr, $resource:expr, $input:expr) => {{
        input_ref_list!($resource, $input)
            .iter()
            .map(|resource_ref| $ref_to_resource.get(resource_ref).unwrap())
            .collect::<Vec<_>>()
    }};
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceStateV2 {
    pub environments: HashMap<String, Vec<LegacyResource>>,
}
impl From<ResourceStateV2> for ResourceStateV3 {
    fn from(state: ResourceStateV2) -> Self {
        let mut environments = HashMap::new();

        for (environment_name, resources) in state.environments {
            let mut id_to_resource: HashMap<ResourceId, RobloxResource> = HashMap::new();
            let mut ref_to_resource: HashMap<ResourceRef, RobloxResource> = HashMap::new();

            let resource_graph = LegacyResourceGraph::new(&resources);
            let resource_order = resource_graph.get_topological_order().unwrap();

            for resource_ref in resource_order {
                let resource = resource_graph.get_resource_from_ref(&resource_ref).unwrap();
                let new_resource: Option<RobloxResource> = match resource.resource_type.as_str() {
                    "experience" => RobloxResource::existing(
                        &format!("experience_{}", resource.id),
                        RobloxInputs::Experience(ExperienceInputs {
                            group_id: input_value!(resource, "groupId"),
                        }),
                        RobloxOutputs::Experience(ExperienceOutputs {
                            asset_id: output_value!(resource, "assetId"),
                            start_place_id: output_value!(resource, "startPlaceId"),
                        }),
                        &[],
                    )
                    .into(),
                    "experienceConfiguration" => RobloxResource::existing(
                        &format!("experienceConfiguration_{}", resource.id),
                        RobloxInputs::ExperienceConfiguration(input_value!(
                            resource,
                            "configuration"
                        )),
                        RobloxOutputs::ExperienceConfiguration,
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "experienceActivation" => RobloxResource::existing(
                        &format!("experienceActivation_{}", resource.id),
                        RobloxInputs::ExperienceActivation(ExperienceActivationInputs {
                            is_active: input_value!(resource, "isActive"),
                        }),
                        RobloxOutputs::ExperienceActivation,
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "experienceIcon" => RobloxResource::existing(
                        "experienceIcon_singleton",
                        RobloxInputs::ExperienceIcon(FileInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                        }),
                        RobloxOutputs::ExperienceIcon(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "experienceThumbnail" => RobloxResource::existing(
                        &format!("experienceThumbnail_{}", resource.id),
                        RobloxInputs::ExperienceThumbnail(FileInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                        }),
                        RobloxOutputs::ExperienceThumbnail(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "experienceThumbnailOrder" => {
                        let thumbnails = dependency_list!(ref_to_resource, resource, "assetIds");
                        RobloxResource::existing(
                            &format!("experienceThumbnailOrder_{}", resource.id),
                            RobloxInputs::ExperienceThumbnailOrder,
                            RobloxOutputs::ExperienceThumbnailOrder,
                            &thumbnails,
                        )
                        .add_dependency(dependency!(ref_to_resource, resource, "experienceId"))
                        .clone()
                        .into()
                    }
                    "place" => RobloxResource::existing(
                        &format!("place_{}", resource.id),
                        RobloxInputs::Place(PlaceInputs {
                            is_start: resource.id == "start",
                        }),
                        RobloxOutputs::Place(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "placeFile" => RobloxResource::existing(
                        &format!("placeFile_{}", resource.id),
                        RobloxInputs::PlaceFile(FileInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                        }),
                        RobloxOutputs::PlaceFile(PlaceFileOutputs {
                            version: output_value!(resource, "version"),
                        }),
                        &[dependency!(ref_to_resource, resource, "assetId")],
                    )
                    .into(),
                    "placeConfiguration" => RobloxResource::existing(
                        &format!("placeConfiguration_{}", resource.id),
                        RobloxInputs::PlaceConfiguration(input_value!(resource, "configuration")),
                        RobloxOutputs::PlaceConfiguration,
                        &[dependency!(ref_to_resource, resource, "assetId")],
                    )
                    .into(),
                    "socialLink" => RobloxResource::existing(
                        &format!("socialLink_{}", resource.id),
                        RobloxInputs::SocialLink(SocialLinkInputs {
                            title: input_value!(resource, "title"),
                            url: input_value!(resource, "url"),
                            link_type: input_value!(resource, "linkType"),
                        }),
                        RobloxOutputs::SocialLink(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "developerProduct" => {
                        let mut new_resource = RobloxResource::existing(
                            &format!("product_{}", resource.id),
                            RobloxInputs::Product(ProductInputs {
                                name: input_value!(resource, "name"),
                                description: input_value!(resource, "description"),
                                price: input_value!(resource, "price"),
                            }),
                            RobloxOutputs::Product(ProductOutputs {
                                asset_id: output_value!(resource, "assetId"),
                                product_id: output_value!(resource, "productId"),
                            }),
                            &[dependency!(ref_to_resource, resource, "experienceId")],
                        );
                        if let Some(icon_asset) =
                            optional_dependency!(ref_to_resource, resource, "iconAssetId")
                        {
                            new_resource.add_dependency(icon_asset).clone().into()
                        } else {
                            new_resource.into()
                        }
                    }
                    "developerProductIcon" => RobloxResource::existing(
                        &format!("productIcon_{}", resource.id),
                        RobloxInputs::ProductIcon(FileInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                        }),
                        RobloxOutputs::ProductIcon(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "gamePass" => {
                        let icon_resource = resource_graph.get_resource_from_ref(&(
                            "gamePassIcon".to_owned(),
                            resource.clone().id,
                        ));
                        RobloxResource::existing(
                            &format!("pass_{}", resource.id),
                            RobloxInputs::Pass(PassInputs {
                                name: input_value!(resource, "name"),
                                description: input_value!(resource, "description"),
                                price: input_value!(resource, "price"),
                                icon_file_path: input_value!(resource, "iconFilePath"),
                                icon_file_hash: if let Some(icon_resource) = &icon_resource {
                                    input_value!(icon_resource, "fileHash")
                                } else {
                                    "unknown".to_owned()
                                },
                            }),
                            RobloxOutputs::Pass(PassOutputs {
                                asset_id: output_value!(resource, "assetId"),
                                icon_asset_id: if let Some(icon_resource) = icon_resource {
                                    output_value!(icon_resource, "assetId")
                                } else {
                                    output_value!(resource, "initialIconAssetId")
                                },
                            }),
                            &[dependency!(ref_to_resource, resource, "startPlaceId")],
                        )
                        .into()
                    }
                    "gamePassIcon" => None,
                    "badge" => RobloxResource::existing(
                        &format!("badge_{}", resource.id),
                        RobloxInputs::Badge(BadgeInputs {
                            name: input_value!(resource, "name"),
                            description: input_value!(resource, "description"),
                            enabled: input_value!(resource, "enabled"),
                            icon_file_path: input_value!(resource, "iconFilePath"),
                        }),
                        RobloxOutputs::Badge(AssetWithInitialIconOutputs {
                            asset_id: output_value!(resource, "assetId"),
                            initial_icon_asset_id: output_value!(resource, "initialIconAssetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "experienceId")],
                    )
                    .into(),
                    "badgeIcon" => RobloxResource::existing(
                        &format!("badgeIcon_{}", resource.id),
                        RobloxInputs::BadgeIcon(FileInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                        }),
                        RobloxOutputs::BadgeIcon(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[dependency!(ref_to_resource, resource, "badgeId")],
                    )
                    .into(),
                    "imageAsset" => RobloxResource::existing(
                        &format!("asset_{}", resource.id),
                        RobloxInputs::ImageAsset(FileWithGroupIdInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                            group_id: input_value!(resource, "groupId"),
                        }),
                        RobloxOutputs::ImageAsset(ImageAssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                            decal_asset_id: output_value!(resource, "decalAssetId"),
                        }),
                        &[],
                    )
                    .into(),
                    "audioAsset" => RobloxResource::existing(
                        &format!("asset_{}", resource.id),
                        RobloxInputs::AudioAsset(FileWithGroupIdInputs {
                            file_path: input_value!(resource, "filePath"),
                            file_hash: input_value!(resource, "fileHash"),
                            group_id: input_value!(resource, "groupId"),
                        }),
                        RobloxOutputs::AudioAsset(AssetOutputs {
                            asset_id: output_value!(resource, "assetId"),
                        }),
                        &[],
                    )
                    .into(),
                    "assetAlias" => RobloxResource::existing(
                        &format!("assetAlias_{}", resource.id),
                        RobloxInputs::AssetAlias(AssetAliasInputs {
                            name: input_value!(resource, "name"),
                        }),
                        RobloxOutputs::AssetAlias(AssetAliasOutputs {
                            name: output_value!(resource, "name"),
                        }),
                        &[
                            dependency!(ref_to_resource, resource, "experienceId"),
                            dependency!(ref_to_resource, resource, "assetId"),
                        ],
                    )
                    .into(),
                    _ => None,
                };

                if let Some(new_resource) = new_resource {
                    id_to_resource.insert(new_resource.get_id(), new_resource.clone());
                    ref_to_resource.insert(resource.get_ref(), new_resource);
                }
            }

            environments.insert(
                environment_name,
                id_to_resource
                    .values()
                    .map(|resource| serde_yaml::to_value(resource).unwrap())
                    .collect::<Vec<_>>(),
            );
        }

        ResourceStateV3 { environments }
    }
}
