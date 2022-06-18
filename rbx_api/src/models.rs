use std::{clone::Clone, fmt, str};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;

pub type AssetId = u64;

pub const DEFAULT_PLACE_NAME: &str = "Untitled Game";

#[derive(Deserialize, Debug)]
pub struct RobloxApiErrorResponse {
    // There are some other possible properties but we currently have no use for them so they are not
    // included

    // Most error models have a `message` property
    #[serde(alias = "Message")]
    pub message: Option<String>,

    // Some error models (500) have a `title` property instead
    #[serde(alias = "Title")]
    pub title: Option<String>,

    // Some error models on older APIs have an errors array
    #[serde(alias = "Errors")]
    pub errors: Option<Vec<RobloxApiErrorResponse>>,

    // Some errors return a `success` property which can be used to check for errors
    #[serde(alias = "Success")]
    pub success: Option<bool>,
}

impl RobloxApiErrorResponse {
    pub fn reason(self) -> Option<String> {
        if let Some(message) = self.message {
            Some(message)
        } else if let Some(title) = self.title {
            Some(title)
        } else if let Some(errors) = self.errors {
            for error in errors {
                if let Some(message) = error.reason() {
                    return Some(message);
                }
            }
            None
        } else {
            None
        }
    }

    pub fn reason_or_status_code(self, status_code: StatusCode) -> String {
        self.reason()
            .unwrap_or_else(|| format!("Unknown error ({})", status_code))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateExperienceResponse {
    pub universe_id: AssetId,
    pub root_place_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CreatorType {
    User,
    Group,
}
impl fmt::Display for CreatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CreatorType::User => "User",
                CreatorType::Group => "Group",
            }
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExperienceResponse {
    pub root_place_id: AssetId,
    pub is_active: bool,
    pub creator_type: CreatorType,
    pub creator_target_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePlaceResponse {
    pub place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaceResponse {
    pub id: AssetId,
    pub current_saved_version: u32,
    pub name: String,
    pub description: String,
    pub max_player_count: u32,
    pub allow_copying: bool,
    pub social_slot_type: SocialSlotType,
    pub custom_social_slots_count: Option<u32>,
    pub is_root_place: bool,
}

impl From<GetPlaceResponse> for PlaceConfigurationModel {
    fn from(response: GetPlaceResponse) -> Self {
        PlaceConfigurationModel {
            name: response.name,
            description: response.description,
            max_player_count: response.max_player_count,
            allow_copying: response.allow_copying,
            social_slot_type: response.social_slot_type,
            custom_social_slots_count: response.custom_social_slots_count,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovePlaceResponse {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlacesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListPlaceResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaceResponse {
    pub id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageResponse {
    pub target_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeveloperProductResponse {
    pub id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeveloperProductsResponse {
    pub developer_products: Vec<ListDeveloperProductResponseItem>,
    pub final_page: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeveloperProductResponseItem {
    pub product_id: AssetId,
    pub developer_product_id: AssetId,
    pub name: String,
    pub description: Option<String>,
    pub icon_image_asset_id: Option<AssetId>,
    pub price_in_robux: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDeveloperProductResponse {
    pub id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGamePassesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListGamePassResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGamePassResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetGamePassResponse {
    pub target_id: AssetId,
    pub name: String,
    pub description: String,
    pub icon_image_asset_id: AssetId,
    pub price_in_robux: Option<u32>,
}

pub struct CreateGamePassResponse {
    pub asset_id: AssetId,
    pub icon_asset_id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateBadgeResponse {
    pub id: AssetId,
    pub icon_image_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBadgesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListBadgeResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListBadgeResponse {
    pub id: AssetId,
    pub name: String,
    pub description: String,
    pub icon_image_id: AssetId,
    pub enabled: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAssetAliasesResponse {
    pub aliases: Vec<GetAssetAliasResponse>,
    pub final_page: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAssetAliasResponse {
    pub name: String,
    pub target_id: AssetId,
    pub asset: GetAssetResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAssetResponse {
    pub type_id: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateImageAssetResponse {
    pub asset_id: AssetId,
    pub backing_asset_id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAudioAssetResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetGameIconsResponse {
    pub data: Vec<GetThumbnailResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetExperienceThumbnailsResponse {
    pub data: Vec<GetExperienceThumbnailResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetExperienceThumbnailResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetThumbnailResponse {
    pub target_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum SocialLinkType {
    Facebook,
    Twitter,
    YouTube,
    Twitch,
    Discord,
    RobloxGroup,
    Guilded,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSocialLinkResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSocialLinksResponse {
    pub data: Vec<GetSocialLinkResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSocialLinkResponse {
    pub id: AssetId,
    pub title: String,
    pub url: Url,
    #[serde(rename = "type")]
    pub link_type: SocialLinkType,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetQuotasResponse {
    pub quotas: Vec<CreateAssetQuota>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum QuotaDuration {
    Month,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssetQuota {
    pub duration: QuotaDuration,
    pub usage: u32,
    pub capacity: u32,
    pub expiration_time: Option<String>,
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

#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum AssetTypeId {
    Image = 1,
    TShirt = 2,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Hat = 8,
    Place = 9,
    Model = 10,
    Shirt = 11,
    Pants = 12,
    Decal = 13,
    Head = 17,
    Face = 18,
    Gear = 19,
    Badge = 21,
    Animation = 24,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    Package = 32,
    GamePass = 34,
    Plugin = 38,
    MeshPart = 40,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    ClimbAnimation = 48,
    DeathAnimation = 49,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    PoseAnimation = 56,
    EarAccessory = 57,
    EyeAccessory = 58,
    EmoteAnimation = 61,
    Video = 62,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
}
impl fmt::Display for AssetTypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap(),)
    }
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
pub enum SocialSlotType {
    Automatic,
    Empty,
    Custom,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceConfigurationModel {
    pub name: String,
    pub description: String,
    pub max_player_count: u32,
    pub allow_copying: bool,
    pub social_slot_type: SocialSlotType,
    pub custom_social_slots_count: Option<u32>,
}

impl Default for PlaceConfigurationModel {
    fn default() -> Self {
        PlaceConfigurationModel {
            name: DEFAULT_PLACE_NAME.to_owned(),
            description: "Created with Mantle".to_owned(),
            max_player_count: 50,
            allow_copying: false,
            social_slot_type: SocialSlotType::Automatic,
            custom_social_slots_count: None,
        }
    }
}

pub enum PlaceFileFormat {
    Xml,
    Binary,
}
