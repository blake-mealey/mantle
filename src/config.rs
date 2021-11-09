use crate::roblox_api::{
    DeployMode, ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
    ExperienceConfigurationModel, ExperienceGenre, ExperiencePermissionsModel,
    ExperiencePlayableDevice, PlaceConfigurationModel, SocialSlotType,
};
use rusoto_core::Region;
use serde::Deserialize;
use std::{collections::HashMap, default, fmt, fs, path::Path, str};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "Vec::new")]
    pub deployments: Vec<DeploymentConfig>,

    #[serde(default)]
    pub templates: TemplateConfig,

    #[serde(default)]
    pub state: StateConfig,
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
            "{}/{}/{}.rocat-state.yml",
            self.region.name(),
            self.bucket,
            self.key
        )
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentConfig {
    pub name: String,

    #[serde(default = "Vec::new")]
    pub branches: Vec<String>,

    #[serde(default)]
    pub deploy_mode: DeployMode,

    #[serde(default)]
    pub tag_commit: bool,

    pub experience_id: u64,

    pub place_ids: HashMap<String, u64>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfig {
    pub experience: Option<ExperienceTemplateConfig>,

    #[serde(default = "HashMap::new")]
    pub places: HashMap<String, PlaceTemplateConfig>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GenreConfig {
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

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayabilityConfig {
    Private,
    Public,
    Friends,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AvatarTypeConfig {
    R6,
    R15,
    PlayerChoice,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperProductConifg {
    pub name: String,
    pub price: u32,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTemplateConfig {
    // basic info
    pub genre: Option<GenreConfig>,
    pub playable_devices: Option<Vec<ExperiencePlayableDevice>>,
    pub icon: Option<String>,
    pub thumbnails: Option<Vec<String>>,

    // permissions
    pub playability: Option<PlayabilityConfig>,

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
    pub avatar_type: Option<AvatarTypeConfig>,
    pub avatar_animation_type: Option<ExperienceAnimationType>,
    pub avatar_collision_type: Option<ExperienceCollisionType>,
    // avatar_asset_overrides: Option<HashMap<String, u64>>,    // TODO: figure out api
    // avatar_scale_constraints: Option<HashMap<String, (f32, f32)>>,   // TODO: figure out api

    // other
    // is_archived: Option<bool>,
    pub developer_products: Option<HashMap<String, DeveloperProductConifg>>,
}

impl From<&ExperienceTemplateConfig> for ExperienceConfigurationModel {
    fn from(config: &ExperienceTemplateConfig) -> Self {
        ExperienceConfigurationModel {
            genre: match config.genre {
                Some(GenreConfig::All) => Some(ExperienceGenre::All),
                Some(GenreConfig::Adventure) => Some(ExperienceGenre::Adventure),
                Some(GenreConfig::Building) => Some(ExperienceGenre::Tutorial),
                Some(GenreConfig::Comedy) => Some(ExperienceGenre::Funny),
                Some(GenreConfig::Fighting) => Some(ExperienceGenre::Ninja),
                Some(GenreConfig::Fps) => Some(ExperienceGenre::Fps),
                Some(GenreConfig::Horror) => Some(ExperienceGenre::Scary),
                Some(GenreConfig::Medieval) => Some(ExperienceGenre::Fantasy),
                Some(GenreConfig::Military) => Some(ExperienceGenre::War),
                Some(GenreConfig::Naval) => Some(ExperienceGenre::Pirate),
                Some(GenreConfig::Rpg) => Some(ExperienceGenre::Rpg),
                Some(GenreConfig::SciFi) => Some(ExperienceGenre::SciFi),
                Some(GenreConfig::Sports) => Some(ExperienceGenre::Sports),
                Some(GenreConfig::TownAndCity) => Some(ExperienceGenre::TownAndCity),
                Some(GenreConfig::Western) => Some(ExperienceGenre::WildWest),
                None => None,
            },
            playable_devices: config
                .playable_devices
                .as_ref()
                .map(|devices| devices.to_vec()),

            is_friends_only: match config.playability {
                Some(PlayabilityConfig::Friends) => Some(true),
                Some(PlayabilityConfig::Public) => Some(false),
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
                Some(AvatarTypeConfig::R6) => Some(ExperienceAvatarType::MorphToR6),
                Some(AvatarTypeConfig::R15) => Some(ExperienceAvatarType::MorphToR15),
                Some(AvatarTypeConfig::PlayerChoice) => Some(ExperienceAvatarType::PlayerChoice),
                None => None,
            },
            universe_animation_type: config.avatar_animation_type,
            universe_collision_type: config.avatar_collision_type,
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServerFillConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTemplateConfig {
    pub file: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_player_count: Option<u32>,
    pub allow_copying: Option<bool>,
    pub server_fill: Option<ServerFillConfig>,
}

impl From<PlaceTemplateConfig> for PlaceConfigurationModel {
    fn from(config: PlaceTemplateConfig) -> Self {
        PlaceConfigurationModel {
            name: config.name.clone(),
            description: config.description.clone(),
            max_player_count: config.max_player_count,
            allow_copying: config.allow_copying,
            social_slot_type: match config.server_fill {
                Some(ServerFillConfig::RobloxOptimized) => Some(SocialSlotType::Automatic),
                Some(ServerFillConfig::Maximum) => Some(SocialSlotType::Empty),
                Some(ServerFillConfig::ReservedSlots(_)) => Some(SocialSlotType::Custom),
                None => None,
            },
            custom_social_slot_count: match config.server_fill {
                Some(ServerFillConfig::ReservedSlots(count)) => Some(count),
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
