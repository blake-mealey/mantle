use std::collections::HashMap;

use rbx_api::models::{ExperienceConfigurationModel, PlaceConfigurationModel};
use serde::{Deserialize, Serialize};
use serde_yaml::{to_value, Mapping, Value};

use super::{
    legacy_resources::{Input, LegacyResource},
    v2::ResourceStateV2,
};

macro_rules! missing_value {
    ($map:expr, $key:expr) => {{
        !$map.contains_key(&Value::String($key.to_owned()))
            || $map.get(&Value::String($key.to_owned())).unwrap().is_null()
    }};
}

macro_rules! provide_default {
    ($map:expr, $key:expr, $default:expr) => {{
        if missing_value!($map, $key) {
            $map.insert(Value::String($key.to_owned()), to_value($default).unwrap());
            true
        } else {
            false
        }
    }};
}

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
                        .insert("groupId".to_owned(), Input::Value(Value::Null));
                }

                // Resources format change: add missing experience configuration defaults.
                // Previously, the ExperienceConfigurationModel allowed all options to be missing,
                // but the new version requires that all options be set.
                if matches!(r_type, "experienceConfiguration") {
                    if let Input::Value(configuration) =
                        resource.inputs.get_mut("configuration").unwrap()
                    {
                        let default_model = ExperienceConfigurationModel::default();
                        let configuration = configuration.as_mapping_mut().unwrap();
                        provide_default!(configuration, "genre", default_model.genre);
                        provide_default!(
                            configuration,
                            "playableDevices",
                            default_model.playable_devices
                        );
                        provide_default!(
                            configuration,
                            "allowPrivateServers",
                            default_model.allow_private_servers
                        );
                        provide_default!(configuration, "isForSale", default_model.is_for_sale);
                        provide_default!(
                            configuration,
                            "studioAccessToApisAllowed",
                            default_model.studio_access_to_apis_allowed
                        );
                        if !provide_default!(
                            configuration,
                            "studioAccessToApisAllowed",
                            default_model.studio_access_to_apis_allowed
                        ) {
                            let permissions = match configuration
                                .get_mut(&Value::String("permissions".to_owned()))
                                .unwrap()
                            {
                                Value::Null => {
                                    configuration.insert(
                                        Value::String("permissions".to_owned()),
                                        Value::Mapping(Mapping::new()),
                                    );
                                    configuration
                                        .get_mut(&Value::String("permissions".to_owned()))
                                        .unwrap()
                                        .as_mapping_mut()
                                        .unwrap()
                                }
                                Value::Mapping(v) => v,
                                _ => unreachable!(),
                            };
                            provide_default!(
                                permissions,
                                "IsThirdPartyPurchaseAllowed",
                                default_model.permissions.is_third_party_purchase_allowed
                            );
                            provide_default!(
                                permissions,
                                "IsThirdPartyTeleportAllowed",
                                default_model.permissions.is_third_party_teleport_allowed
                            );
                        }
                        provide_default!(
                            configuration,
                            "universeAvatarType",
                            default_model.universe_avatar_type
                        );
                        provide_default!(
                            configuration,
                            "universeAnimationType",
                            default_model.universe_animation_type
                        );
                        provide_default!(
                            configuration,
                            "universeCollisionType",
                            default_model.universe_collision_type
                        );
                        provide_default!(
                            configuration,
                            "universeAvatarMinScales",
                            default_model.universe_avatar_min_scales
                        );
                        provide_default!(
                            configuration,
                            "universeAvatarMaxScales",
                            default_model.universe_avatar_max_scales
                        );
                        provide_default!(
                            configuration,
                            "universeAvatarAssetOverrides",
                            default_model.universe_avatar_asset_overrides
                        );
                        provide_default!(configuration, "isArchived", default_model.is_archived);
                    }
                }

                // Resources format change: add missing place configuration defaults. Previously,
                // the PlaceConfigurationModel allowed all options to be missing, but the new
                // version requires that all options be set.
                if matches!(r_type, "placeConfiguration") {
                    if let Input::Value(configuration) =
                        resource.inputs.get_mut("configuration").unwrap()
                    {
                        let default_model = PlaceConfigurationModel::default();
                        let configuration = configuration.as_mapping_mut().unwrap();
                        provide_default!(configuration, "name", default_model.name);
                        provide_default!(configuration, "description", default_model.description);
                        provide_default!(
                            configuration,
                            "maxPlayerCount",
                            default_model.max_player_count
                        );
                        provide_default!(
                            configuration,
                            "allowCopying",
                            default_model.allow_copying
                        );
                        provide_default!(
                            configuration,
                            "socialSlotType",
                            default_model.social_slot_type
                        );
                    }
                }

                // Resources format change: add missing "description" defaults. Previously, these
                // resources allowed "description" to be missing, but the new version requires that
                // it is set.
                if matches!(r_type, "developerProduct" | "gamePass" | "badge") {
                    if let None | Some(Input::Value(Value::Null)) =
                        resource.inputs.get("description")
                    {
                        resource.inputs.insert(
                            "description".to_owned(),
                            Input::Value(Value::String("".to_owned())),
                        );
                    };
                }
            }
        }

        println!(
            "{}",
            serde_yaml::to_string(&ResourceStateV2 {
                environments: environments.clone()
            })
            .unwrap()
        );

        ResourceStateV2 { environments }
    }
}
