use std::{
    collections::HashMap,
    default, fmt, fs,
    path::{Path, PathBuf},
    str,
};

use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use url::Url;
use yansi::Paint;

use super::{
    logger,
    roblox_api::{
        AssetTypeId, ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
        ExperienceConfigurationModel, ExperienceGenre, ExperiencePlayableDevice,
        PlaceConfigurationModel, SocialSlotType,
    },
    roblox_resource_manager::AssetId,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub owner: OwnerConfig,

    #[serde(default)]
    pub payments: PaymentsConfig,

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
pub enum PaymentsConfig {
    Owner,
    Personal,
    Group,
}
impl default::Default for PaymentsConfig {
    fn default() -> Self {
        PaymentsConfig::Owner
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

    pub social_links: Option<Vec<SocialLinkTargetConfig>>,

    pub products: Option<HashMap<String, ProductTargetConifg>>,

    pub passes: Option<HashMap<String, PassTargetConfig>>,

    pub badges: Option<HashMap<String, BadgeTargetConfig>>,

    pub assets: Option<Vec<AssetTargetConfig>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialLinkTargetConfig {
    pub title: String,
    pub url: Url,
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

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AvatarScaleConstraintsTargetConfig {
    pub height: Option<Constraint>,
    pub width: Option<Constraint>,
    pub head: Option<Constraint>,
    pub body_type: Option<Constraint>,
    pub proportions: Option<Constraint>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAssetOverridesTargetConfig {
    pub face: Option<AssetId>,
    pub head: Option<AssetId>,
    pub torso: Option<AssetId>,
    pub left_arm: Option<AssetId>,
    pub right_arm: Option<AssetId>,
    pub left_leg: Option<AssetId>,
    pub right_leg: Option<AssetId>,
    #[serde(rename = "tshirt")]
    pub t_shirt: Option<AssetId>,
    pub shirt: Option<AssetId>,
    pub pants: Option<AssetId>,
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
    pub paid_access_price: Option<u32>,
    pub private_server_price: Option<u32>,

    // security
    pub enable_studio_access_to_apis: Option<bool>,
    pub allow_third_party_sales: Option<bool>,
    pub allow_third_party_teleports: Option<bool>,

    // localization: // TODO: localization

    // avatar
    pub avatar_type: Option<AvatarTypeTargetConfig>,
    pub avatar_animation_type: Option<AnimationTypeTargetConfig>,
    pub avatar_collision_type: Option<CollisionTypeTargetConfig>,
    pub avatar_scale_constraints: Option<AvatarScaleConstraintsTargetConfig>,
    pub avatar_asset_overrides: Option<AvatarAssetOverridesTargetConfig>,
}

impl From<&ExperienceTargetConfigurationConfig> for ExperienceConfigurationModel {
    fn from(config: &ExperienceTargetConfigurationConfig) -> Self {
        let mut model = ExperienceConfigurationModel::default();
        if let Some(genre) = &config.genre {
            model.genre = match genre {
                GenreTargetConfig::All => ExperienceGenre::All,
                GenreTargetConfig::Adventure => ExperienceGenre::Adventure,
                GenreTargetConfig::Building => ExperienceGenre::Tutorial,
                GenreTargetConfig::Comedy => ExperienceGenre::Funny,
                GenreTargetConfig::Fighting => ExperienceGenre::Ninja,
                GenreTargetConfig::Fps => ExperienceGenre::Fps,
                GenreTargetConfig::Horror => ExperienceGenre::Scary,
                GenreTargetConfig::Medieval => ExperienceGenre::Fantasy,
                GenreTargetConfig::Military => ExperienceGenre::War,
                GenreTargetConfig::Naval => ExperienceGenre::Pirate,
                GenreTargetConfig::Rpg => ExperienceGenre::Rpg,
                GenreTargetConfig::SciFi => ExperienceGenre::SciFi,
                GenreTargetConfig::Sports => ExperienceGenre::Sports,
                GenreTargetConfig::TownAndCity => ExperienceGenre::TownAndCity,
                GenreTargetConfig::Western => ExperienceGenre::WildWest,
            }
        }
        if let Some(playable_devices) = &config.playable_devices {
            model.playable_devices = playable_devices
                .iter()
                .map(|device| match device {
                    PlayableDeviceTargetConfig::Computer => ExperiencePlayableDevice::Computer,
                    PlayableDeviceTargetConfig::Phone => ExperiencePlayableDevice::Phone,
                    PlayableDeviceTargetConfig::Tablet => ExperiencePlayableDevice::Tablet,
                    PlayableDeviceTargetConfig::Console => ExperiencePlayableDevice::Console,
                })
                .collect();
        }
        if let Some(playability) = &config.playability {
            model.is_friends_only = match playability {
                PlayabilityTargetConfig::Friends => Some(true),
                PlayabilityTargetConfig::Public => Some(false),
                PlayabilityTargetConfig::Private => None,
            }
        }
        model.is_for_sale = config.clone().paid_access_price.is_some();
        model.price = config.paid_access_price;
        model.allow_private_servers = config.private_server_price.is_some();
        model.private_server_price = config.private_server_price;
        if let Some(enable_studio_access_to_apis) = config.enable_studio_access_to_apis {
            model.studio_access_to_apis_allowed = enable_studio_access_to_apis;
        }
        if let Some(allow_third_party_sales) = config.allow_third_party_sales {
            model.permissions.is_third_party_purchase_allowed = allow_third_party_sales;
        }
        if let Some(allow_third_party_teleports) = config.allow_third_party_teleports {
            model.permissions.is_third_party_teleport_allowed = allow_third_party_teleports;
        }
        if let Some(avatar_type) = &config.avatar_type {
            model.universe_avatar_type = match avatar_type {
                AvatarTypeTargetConfig::R6 => ExperienceAvatarType::MorphToR6,
                AvatarTypeTargetConfig::R15 => ExperienceAvatarType::MorphToR15,
                AvatarTypeTargetConfig::PlayerChoice => ExperienceAvatarType::PlayerChoice,
            }
        }
        if let Some(avatar_animation_type) = &config.avatar_animation_type {
            model.universe_animation_type = match avatar_animation_type {
                AnimationTypeTargetConfig::Standard => ExperienceAnimationType::Standard,
                AnimationTypeTargetConfig::PlayerChoice => ExperienceAnimationType::PlayerChoice,
            }
        }
        if let Some(avatar_collision_type) = &config.avatar_collision_type {
            model.universe_collision_type = match avatar_collision_type {
                CollisionTypeTargetConfig::OuterBox => ExperienceCollisionType::OuterBox,
                CollisionTypeTargetConfig::InnerBox => ExperienceCollisionType::InnerBox,
            }
        }
        if let Some(constraints) = &config.avatar_scale_constraints {
            if let Some(height) = constraints.height.and_then(|c| c.min) {
                model.universe_avatar_min_scales.height = height.to_string();
            }
            if let Some(width) = constraints.width.and_then(|c| c.min) {
                model.universe_avatar_min_scales.width = width.to_string();
            }
            if let Some(head) = constraints.head.and_then(|c| c.min) {
                model.universe_avatar_min_scales.head = head.to_string();
            }
            if let Some(body_type) = constraints.body_type.and_then(|c| c.min) {
                model.universe_avatar_min_scales.body_type = body_type.to_string();
            }
            if let Some(proportions) = constraints.proportions.and_then(|c| c.min) {
                model.universe_avatar_min_scales.proportion = proportions.to_string();
            }

            if let Some(height) = constraints.height.and_then(|c| c.max) {
                model.universe_avatar_max_scales.height = height.to_string();
            }
            if let Some(width) = constraints.width.and_then(|c| c.max) {
                model.universe_avatar_max_scales.width = width.to_string();
            }
            if let Some(head) = constraints.head.and_then(|c| c.max) {
                model.universe_avatar_max_scales.head = head.to_string();
            }
            if let Some(body_type) = constraints.body_type.and_then(|c| c.max) {
                model.universe_avatar_max_scales.body_type = body_type.to_string();
            }
            if let Some(proportions) = constraints.proportions.and_then(|c| c.max) {
                model.universe_avatar_max_scales.proportion = proportions.to_string();
            }
        }
        if let Some(avatar_asset_overrides) = &config.avatar_asset_overrides {
            for override_model in model.universe_avatar_asset_overrides.iter_mut() {
                if let Some(override_config) = match override_model.asset_type_id {
                    AssetTypeId::Face => avatar_asset_overrides.face,
                    AssetTypeId::Head => avatar_asset_overrides.head,
                    AssetTypeId::Torso => avatar_asset_overrides.torso,
                    AssetTypeId::LeftArm => avatar_asset_overrides.left_arm,
                    AssetTypeId::RightArm => avatar_asset_overrides.right_arm,
                    AssetTypeId::LeftLeg => avatar_asset_overrides.left_leg,
                    AssetTypeId::RightLeg => avatar_asset_overrides.right_leg,
                    AssetTypeId::TShirt => avatar_asset_overrides.t_shirt,
                    AssetTypeId::Shirt => avatar_asset_overrides.shirt,
                    AssetTypeId::Pants => avatar_asset_overrides.pants,
                    _ => None,
                } {
                    override_model.is_player_choice = false;
                    override_model.asset_id = Some(override_config);
                }
            }
        }
        model
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
        let mut model = PlaceConfigurationModel::default();
        if let Some(name) = config.name {
            model.name = name;
        }
        if let Some(description) = config.description {
            model.description = description;
        }
        if let Some(max_player_count) = config.max_player_count {
            model.max_player_count = max_player_count;
        }
        if let Some(allow_copying) = config.allow_copying {
            model.allow_copying = allow_copying;
        }
        if let Some(server_fill) = config.server_fill {
            model.social_slot_type = match server_fill {
                ServerFillTargetConfig::RobloxOptimized => SocialSlotType::Automatic,
                ServerFillTargetConfig::Maximum => SocialSlotType::Empty,
                ServerFillTargetConfig::ReservedSlots(_) => SocialSlotType::Custom,
            };
            model.custom_social_slots_count = match server_fill {
                ServerFillTargetConfig::ReservedSlots(count) => Some(count),
                _ => None,
            }
        }
        model
    }
}

fn parse_project_path(project: Option<&str>) -> Result<(PathBuf, PathBuf), String> {
    let project = project.unwrap_or(".");
    let project_path = Path::new(project).to_owned();

    let (project_dir, config_file) = if project_path.is_dir() {
        (project_path.clone(), project_path.join("mantle.yml"))
    } else if project_path.is_file() {
        (project_path.parent().unwrap().into(), project_path)
    } else {
        return Err(format!("Unable to load project path: {}", project));
    };

    if config_file.exists() {
        return Ok((project_dir, config_file));
    }

    Err(format!("Config file {} not found", config_file.display()))
}

fn load_config_file(config_file: &Path) -> Result<Config, String> {
    let data = fs::read_to_string(config_file).map_err(|e| {
        format!(
            "Unable to read config file: {}\n\t{}",
            config_file.display(),
            e
        )
    })?;

    serde_yaml::from_str::<Config>(&data).map_err(|e| {
        format!(
            "Unable to parse config file {}\n\t{}",
            config_file.display(),
            e
        )
    })
}

pub fn load_project_config(project: Option<&str>) -> Result<(PathBuf, Config), String> {
    let (project_path, config_path) = parse_project_path(project)?;
    let config = load_config_file(&config_path)?;

    logger::log(format!(
        "Loaded config file {}",
        Paint::cyan(config_path.display())
    ));

    Ok((project_path, config))
}
