use serde::{Deserialize, Serialize};

use crate::models::{AssetId, AssetTypeId, CreatorType};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExperienceResponse {
    pub universe_id: AssetId,
    pub root_place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExperienceResponse {
    pub root_place_id: AssetId,
    pub is_active: bool,
    pub creator_type: CreatorType,
    pub creator_target_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceConfigurationModel {
    pub genre: ExperienceGenre,
    pub playable_devices: Vec<ExperiencePlayableDevice>,
    pub is_friends_only: Option<bool>,

    #[serde(default)]
    pub allow_private_servers: bool,
    pub private_server_price: Option<u32>,
    pub is_for_sale: bool,
    pub price: Option<u32>,

    #[serde(default)]
    pub studio_access_to_apis_allowed: bool,
    #[serde(default)]
    pub permissions: ExperiencePermissionsModel,

    pub universe_avatar_type: ExperienceAvatarType,
    pub universe_animation_type: ExperienceAnimationType,
    pub universe_collision_type: ExperienceCollisionType,
    #[serde(default = "default_min_scales")]
    pub universe_avatar_min_scales: ExperienceAvatarScales,
    #[serde(default = "default_max_scales")]
    pub universe_avatar_max_scales: ExperienceAvatarScales,
    #[serde(default = "default_asset_overrides")]
    pub universe_avatar_asset_overrides: Vec<ExperienceAvatarAssetOverride>,

    pub is_archived: bool,
}

fn default_min_scales() -> ExperienceAvatarScales {
    ExperienceConfigurationModel::default().universe_avatar_min_scales
}

fn default_max_scales() -> ExperienceAvatarScales {
    ExperienceConfigurationModel::default().universe_avatar_max_scales
}

fn default_asset_overrides() -> Vec<ExperienceAvatarAssetOverride> {
    ExperienceConfigurationModel::default().universe_avatar_asset_overrides
}

impl Default for ExperienceConfigurationModel {
    fn default() -> Self {
        ExperienceConfigurationModel {
            genre: ExperienceGenre::All,
            playable_devices: vec![
                ExperiencePlayableDevice::Computer,
                ExperiencePlayableDevice::Phone,
                ExperiencePlayableDevice::Tablet,
            ],
            is_friends_only: Some(true),

            allow_private_servers: false,
            private_server_price: None,
            is_for_sale: false,
            price: None,

            studio_access_to_apis_allowed: false,
            permissions: ExperiencePermissionsModel {
                is_third_party_purchase_allowed: false,
                is_third_party_teleport_allowed: false,
            },

            universe_avatar_type: ExperienceAvatarType::MorphToR15,
            universe_animation_type: ExperienceAnimationType::PlayerChoice,
            universe_collision_type: ExperienceCollisionType::OuterBox,
            universe_avatar_min_scales: ExperienceAvatarScales {
                height: 0.9.to_string(),
                width: 0.7.to_string(),
                head: 0.95.to_string(),
                body_type: 0.0.to_string(),
                proportion: 0.0.to_string(),
            },
            universe_avatar_max_scales: ExperienceAvatarScales {
                height: 1.05.to_string(),
                width: 1.0.to_string(),
                head: 1.0.to_string(),
                body_type: 1.0.to_string(),
                proportion: 1.0.to_string(),
            },
            universe_avatar_asset_overrides: vec![
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::Face),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::Head),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::Torso),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::LeftArm),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::RightArm),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::LeftLeg),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::RightLeg),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::TShirt),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::Shirt),
                ExperienceAvatarAssetOverride::player_choice(AssetTypeId::Pants),
            ],

            is_archived: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ExperienceGenre {
    All,
    Adventure,
    Tutorial,
    Funny,
    Ninja,
    #[serde(rename = "FPS")]
    Fps,
    Scary,
    Fantasy,
    War,
    Pirate,
    #[serde(rename = "RPG")]
    Rpg,
    SciFi,
    Sports,
    TownAndCity,
    WildWest,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum ExperiencePlayableDevice {
    Computer,
    Phone,
    Tablet,
    Console,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ExperienceAvatarType {
    MorphToR6,
    MorphToR15,
    PlayerChoice,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum ExperienceAnimationType {
    Standard,
    PlayerChoice,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum ExperienceCollisionType {
    OuterBox,
    InnerBox,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceAvatarScales {
    pub height: String,
    pub width: String,
    pub head: String,
    pub body_type: String,
    pub proportion: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceAvatarAssetOverride {
    #[serde(rename = "assetTypeID")]
    pub asset_type_id: AssetTypeId,
    pub is_player_choice: bool,
    #[serde(rename = "assetID")]
    pub asset_id: Option<AssetId>,
}
impl ExperienceAvatarAssetOverride {
    pub fn player_choice(asset_type_id: AssetTypeId) -> Self {
        Self {
            asset_type_id,
            is_player_choice: true,
            asset_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExperiencePermissionsModel {
    pub is_third_party_purchase_allowed: bool,
    pub is_third_party_teleport_allowed: bool,
}
