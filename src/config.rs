use crate::{
    resource_manager::AssetId,
    roblox_api::{
        ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
        ExperienceConfigurationModel, ExperienceGenre, ExperiencePermissionsModel,
        ExperiencePlayableDevice, PlaceConfigurationModel, SocialSlotType,
    },
};
use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default, fmt, fs, path::Path, str};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub owner: OwnerConfig,

    #[serde(default = "Vec::new")]
    pub environments: Vec<EnvironmentConfig>,

    pub target: TargetConfig,

    #[serde(default)]
    pub state: StateConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OwnerConfig {
    Personal,
    Group(AssetId),
}
impl default::Default for OwnerConfig {
    fn default() -> Self {
        OwnerConfig::Personal
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StateConfig {
    Local,
    Remote(RemoteStateConfig),
}
impl default::Default for StateConfig {
    fn default() -> Self {
        StateConfig::Local
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStateConfig {
    pub bucket: String,
    pub key: String,
    pub region: Region,
}
impl fmt::Display for RemoteStateConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}.mantle-state.yml",
            self.region.name(),
            self.bucket,
            self.key
        )
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentConfig {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub branches: Vec<String>,

    #[serde(default)]
    pub tag_commit: bool,

    pub overrides: Option<serde_yaml::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetConfig {
    Experience(ExperienceTargetConfig),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTargetConfig {
    pub configuration: Option<ExperienceTargetConfigurationConfig>,

    pub places: Option<HashMap<String, PlaceTargetConfig>>,

    pub products: Option<HashMap<String, ProductTargetConifg>>,

    pub passes: Option<HashMap<String, PassTargetConfig>>,

    pub badges: Option<HashMap<String, BadgeTargetConfig>>,

    pub assets: Option<Vec<AssetTargetConfig>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GenreTargetConfig {
    All,
    Adventure,
    Building,
    Comedy,
    Fighting,
    Fps,
    Horror,
    Medieval,
    Military,
    Naval,
    Rpg,
    SciFi,
    Sports,
    TownAndCity,
    Western,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayabilityTargetConfig {
    Private,
    Public,
    Friends,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AvatarTypeTargetConfig {
    R6,
    R15,
    PlayerChoice,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayableDeviceTargetConfig {
    Computer,
    Phone,
    Tablet,
    Console,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AnimationTypeTargetConfig {
    Standard,
    PlayerChoice,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CollisionTypeTargetConfig {
    OuterBox,
    InnerBox,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductTargetConifg {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub price: u32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassTargetConfig {
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub price: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeTargetConfig {
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AssetTargetConfig {
    File(String),
    FileWithAlias { file: String, name: String },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTargetConfigurationConfig {
    // basic info
    pub genre: Option<GenreTargetConfig>,
    pub playable_devices: Option<Vec<PlayableDeviceTargetConfig>>,
    pub icon: Option<String>,
    pub thumbnails: Option<Vec<String>>,

    // permissions
    pub playability: Option<PlayabilityTargetConfig>,

    // monetization
    // badges: // TODO: create badges
    pub paid_access_price: Option<u32>,
    pub private_server_price: Option<u32>,
    // developer products: // TODO: create developer products

    // security
    pub enable_studio_access_to_apis: Option<bool>,
    pub allow_third_party_sales: Option<bool>,
    pub allow_third_party_teleports: Option<bool>,

    // localization: // TODO: localization

    // avatar
    pub avatar_type: Option<AvatarTypeTargetConfig>,
    pub avatar_animation_type: Option<AnimationTypeTargetConfig>,
    pub avatar_collision_type: Option<CollisionTypeTargetConfig>,
    // avatar_asset_overrides: Option<HashMap<String, u64>>,    // TODO: figure out api
    // avatar_scale_constraints: Option<HashMap<String, (f32, f32)>>,   // TODO: figure out api
}

impl From<&ExperienceTargetConfigurationConfig> for ExperienceConfigurationModel {
    fn from(config: &ExperienceTargetConfigurationConfig) -> Self {
        ExperienceConfigurationModel {
            genre: match config.genre {
                Some(GenreTargetConfig::All) => Some(ExperienceGenre::All),
                Some(GenreTargetConfig::Adventure) => Some(ExperienceGenre::Adventure),
                Some(GenreTargetConfig::Building) => Some(ExperienceGenre::Tutorial),
                Some(GenreTargetConfig::Comedy) => Some(ExperienceGenre::Funny),
                Some(GenreTargetConfig::Fighting) => Some(ExperienceGenre::Ninja),
                Some(GenreTargetConfig::Fps) => Some(ExperienceGenre::Fps),
                Some(GenreTargetConfig::Horror) => Some(ExperienceGenre::Scary),
                Some(GenreTargetConfig::Medieval) => Some(ExperienceGenre::Fantasy),
                Some(GenreTargetConfig::Military) => Some(ExperienceGenre::War),
                Some(GenreTargetConfig::Naval) => Some(ExperienceGenre::Pirate),
                Some(GenreTargetConfig::Rpg) => Some(ExperienceGenre::Rpg),
                Some(GenreTargetConfig::SciFi) => Some(ExperienceGenre::SciFi),
                Some(GenreTargetConfig::Sports) => Some(ExperienceGenre::Sports),
                Some(GenreTargetConfig::TownAndCity) => Some(ExperienceGenre::TownAndCity),
                Some(GenreTargetConfig::Western) => Some(ExperienceGenre::WildWest),
                None => None,
            },
            playable_devices: config.playable_devices.as_ref().map(|devices| {
                devices
                    .iter()
                    .map(|d| match d {
                        PlayableDeviceTargetConfig::Computer => ExperiencePlayableDevice::Computer,
                        PlayableDeviceTargetConfig::Console => ExperiencePlayableDevice::Console,
                        PlayableDeviceTargetConfig::Phone => ExperiencePlayableDevice::Phone,
                        PlayableDeviceTargetConfig::Tablet => ExperiencePlayableDevice::Tablet,
                    })
                    .collect()
            }),

            is_friends_only: match config.playability {
                Some(PlayabilityTargetConfig::Friends) => Some(true),
                Some(PlayabilityTargetConfig::Public) => Some(false),
                _ => None,
            },

            is_for_sale: match config.paid_access_price {
                Some(_) => Some(true),
                _ => None,
            },
            price: config.paid_access_price,
            allow_private_servers: match config.private_server_price {
                Some(_) => Some(true),
                _ => None,
            },
            private_server_price: config.private_server_price,

            studio_access_to_apis_allowed: config.enable_studio_access_to_apis,
            permissions: match (
                config.allow_third_party_sales,
                config.allow_third_party_teleports,
            ) {
                (None, None) => None,
                (allow_third_party_sales, allow_third_party_teleports) => {
                    Some(ExperiencePermissionsModel {
                        is_third_party_purchase_allowed: allow_third_party_sales,
                        is_third_party_teleport_allowed: allow_third_party_teleports,
                    })
                }
            },

            universe_avatar_type: match config.avatar_type {
                Some(AvatarTypeTargetConfig::R6) => Some(ExperienceAvatarType::MorphToR6),
                Some(AvatarTypeTargetConfig::R15) => Some(ExperienceAvatarType::MorphToR15),
                Some(AvatarTypeTargetConfig::PlayerChoice) => {
                    Some(ExperienceAvatarType::PlayerChoice)
                }
                None => None,
            },
            universe_animation_type: match config.avatar_animation_type {
                Some(AnimationTypeTargetConfig::Standard) => {
                    Some(ExperienceAnimationType::Standard)
                }
                Some(AnimationTypeTargetConfig::PlayerChoice) => {
                    Some(ExperienceAnimationType::PlayerChoice)
                }
                None => None,
            },
            universe_collision_type: match config.avatar_collision_type {
                Some(CollisionTypeTargetConfig::InnerBox) => {
                    Some(ExperienceCollisionType::InnerBox)
                }
                Some(CollisionTypeTargetConfig::OuterBox) => {
                    Some(ExperienceCollisionType::OuterBox)
                }
                None => None,
            },

            is_archived: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServerFillTargetConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTargetConfig {
    pub file: Option<String>,
    pub configuration: Option<PlaceTargetConfigurationConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTargetConfigurationConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_player_count: Option<u32>,
    pub allow_copying: Option<bool>,
    pub server_fill: Option<ServerFillTargetConfig>,
}

impl From<PlaceTargetConfigurationConfig> for PlaceConfigurationModel {
    fn from(config: PlaceTargetConfigurationConfig) -> Self {
        PlaceConfigurationModel {
            name: config.name.clone(),
            description: config.description.clone(),
            max_player_count: config.max_player_count,
            allow_copying: config.allow_copying,
            social_slot_type: match config.server_fill {
                Some(ServerFillTargetConfig::RobloxOptimized) => Some(SocialSlotType::Automatic),
                Some(ServerFillTargetConfig::Maximum) => Some(SocialSlotType::Empty),
                Some(ServerFillTargetConfig::ReservedSlots(_)) => Some(SocialSlotType::Custom),
                None => None,
            },
            custom_social_slot_count: match config.server_fill {
                Some(ServerFillTargetConfig::ReservedSlots(count)) => Some(count),
                _ => None,
            },
        }
    }
}

pub fn load_config_file(config_file: &Path) -> Result<Config, String> {
    let data = match fs::read_to_string(config_file) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!(
                "Unable to read config file: {}\n\t{}",
                config_file.display(),
                e
            ))
        }
    };

    match serde_yaml::from_str::<Config>(&data) {
        Ok(v) => Ok(v),
        Err(e) => {
            return Err(format!(
                "Unable to parse config file {}\n\t{}",
                config_file.display(),
                e
            ))
        }
    }
}
