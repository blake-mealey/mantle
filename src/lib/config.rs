use std::{
    collections::HashMap,
    default, fmt, fs,
    path::{Path, PathBuf},
    str,
};

use rusoto_core::Region;
use schemars::JsonSchema;
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

/// # Configuration
#[derive(JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// # Owner
    /// The owner of the resources that will be created. Defaults to `"personal"`.
    #[serde(default)]
    pub owner: OwnerConfig,

    /// # Payments
    /// Where Robux should come from to purchase resources (if `--allow-purchases` is enabled).
    /// Defaults to `"owner"`.
    #[serde(default)]
    pub payments: PaymentsConfig,

    /// # Environments
    /// The list of environments which Mantle can deploy to.
    pub environments: Vec<EnvironmentConfig>,

    /// # Target
    /// Defines the target resource which Mantle will deploy to. Currently Mantle only supports
    /// targeting Experiences, but in the future it will target other types like Plugins and Models.
    pub target: TargetConfig,

    /// # State
    /// Defines how Mantle should manage state files (locally or remotely). Defaults to `"local"`.
    #[serde(default)]
    pub state: StateConfig,
}

/// # Owner
/// If set to `"personal"`, all resources will be created under the authorizer user.
///
/// If set to a group, all resources will be created under the provided group ID.
///
/// ```yml title="Group Example"
/// owner:
///   group: 5723117
/// ```
///
/// ```yml title="Personal Example (Default)"
/// owner: personal
/// ```
#[derive(JsonSchema, Deserialize, Clone)]
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

/// # Payments
/// If set to `"owner"`, all payments will come from the balance of the owner specified by the
/// top-level `owner` field.
///
/// If set to `"personal"`, all payments will come from the balance of the authorized user.
///
/// If set to `"group"`, all payments will come from the balance of the group specified by the
/// top-level `owner` field. Payments can only be set to `"group"` when `owner` is also set to a
/// group because Roblox does not allow groups to pay for purchases of resources outside of
/// themselves.
///
/// ```yml title="Personal Example"
/// payments: personal
/// ```
///
/// ```yml title="Group Example"
/// payments: group
/// ```
///
/// ```yml title="Owner Example (Default)"
/// payments: owner
/// ```
#[derive(JsonSchema, Deserialize, Clone)]
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

/// # State
/// If set to `"local"`, Mantle will save and load its state to and from a local `.mantle-state.yml`
/// file.
///
/// If set to a remote, Mantle will save and load its state to and from a remote file.
///
/// ```yml title="Remote State Example"
/// state:
///   remote:
///     region: [us-west-2]
///     bucket: mantle-states
///     key: pirate-wars
/// ```
///
/// ```yml title="Local State Example (Default)"
/// state: local
/// ```
#[derive(JsonSchema, Deserialize, Clone)]
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

/// # Region
#[derive(JsonSchema, Deserialize)]
#[serde(remote = "Region")]
pub enum RegionRef {
    #[serde(rename = "ap-east-1")]
    ApEast1,
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    #[serde(rename = "ca-central-1")]
    CaCentral1,
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "eu-west-2")]
    EuWest2,
    #[serde(rename = "eu-west-3")]
    EuWest3,
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    #[serde(rename = "me-south-1")]
    MeSouth1,
    #[serde(rename = "sa-east-1")]
    SaEast1,
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-east-2")]
    UsEast2,
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "us-west-2")]
    UsWest2,
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1,
    #[serde(rename = "cn-north-1")]
    CnNorth1,
    #[serde(rename = "cn-northwest-1")]
    CnNorthwest1,
    #[serde(rename = "af-south-1")]
    AfSouth1,
    #[serde(rename = "custom")]
    Custom { name: String, endpoint: String },
}

/// # RemoteState
#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStateConfig {
    /// # Region
    /// The AWS region your S3 bucket is located in.
    #[serde(with = "RegionRef")]
    pub region: Region,
    /// # Bucket
    /// The name of your AWS S3 bucket.
    pub bucket: String,
    /// # Key
    /// The key to use to store your state file. The file will be named with the format
    /// `<key>.mantle-state.yml`.
    pub key: String,
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

/// # Environment
/// ```yml title="Example"
/// environments:
///   - name: staging
///     branches: [dev, dev/*]
///     overrides:
///       configuration:
///         genre: building
///   - name: production
///     branches: [main]
///     targetAccess: public
/// ```
#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentConfig {
    /// # Name
    /// The name of the environment that is used to identify the environment via the `--environment`
    /// flag. Must be unique across all environments.
    pub name: String,

    /// # Branches
    /// An array of file globs to match against Git branches. If the `--environment` flag is not
    /// specified, Mantle will pick the first environment which contains a matching file glob for
    /// the current Git branch. If no environments match, Mantle will exit with a success code.
    #[serde(default)]
    pub branches: Vec<String>,

    /// # Tag Commit
    /// Whether or not to tag the commit with place file versions after successful deployments. It
    /// is recommended to only enable this on your production environment. Tags will be of the
    /// format `<name>-v<version>` where `<name>` is the name of the place and `<version>` is the
    /// place's Roblox version.
    #[serde(default)]
    pub tag_commit: bool,

    /// # Target Name Prefix
    /// Adds a prefix to the target's name configuration. The implementation is dependent on the
    /// target's type. For Experience targets, all place names will be updated with the prefix.
    pub target_name_prefix: Option<TargetNamePrefixConfig>,

    /// # Target Access
    /// Overrides the target's access. The implementation is dependent on the target's type. For
    /// Experience targets, the `configuration.playability` field will be overridden.
    pub target_access: Option<TargetAccessConfig>,

    // TODO: This could break future target types. It is implemented this way in order to support schemars
    /// # Overrides
    /// Environment-specific overrides for the target resource definition. This will override all
    /// configuration, including changes made by the `targetNamePrefix` and `targetAccess` fields.
    pub overrides: Option<TargetOverridesConfig>,
}

/// # TargetNamePrefix
/// If set to `"environmentName"`, the target name prefix will use the format `[<ENVIRONMENT>] `
/// where `<ENVIRONMENT>` is the value of the environment's `name` field in all caps.
///
/// If set to a custom string, the target name prefix will use the user-specified string.
///
/// ```yml title="Environment Name Example"
/// environments:
///   - name: staging
///     targetNamePrefix: environmentName
///   - name: production
/// ```
///
/// ```yml title="Custom Example"
/// environments:
///   - name: staging
///     targetNamePrefix:
///       custom: 'Prefix: '
///   - name: production
/// ```
#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetNamePrefixConfig {
    EnvironmentName,
    Custom(String),
}

/// # TargetAccess
/// If set to `"public"`, the target will be accessible to all Roblox users.
///
/// If set to `"private"`, the target will only be accessible to the authorized user.
///
/// If set to `"friends"`, the target will only be accessible to the authorized user and that user's
/// Roblox friends.
#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetAccessConfig {
    Public,
    Private,
    Friends,
}

/// # Target
/// Currently the only supported target resource type is an Experience.
///
/// ```yml title="Example"
/// target:
///   experience: {}
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetConfig {
    Experience(ExperienceTargetConfig),
}

/// # TargetOverrides
/// Override the target configuration. Should match the type of the target configuration.
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TargetOverridesConfig {
    Experience(ExperienceTargetConfig),
}

/// # Experience
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTargetConfig {
    /// # Configuration
    /// The Experience's Roblox configuration.
    pub configuration: Option<ExperienceTargetConfigurationConfig>,

    /// # Places
    /// The places to create in the experience. There must be at least one place supplied with the
    /// name `"start"`, which will be used as the start place for the experience.
    pub places: Option<HashMap<String, PlaceTargetConfig>>,

    /// # Social Links
    /// A list of social links that will appear on the experience's webpage.
    pub social_links: Option<Vec<SocialLinkTargetConfig>>,

    /// # Products
    /// Products that can be purchased within your experience for Robux.
    pub products: Option<HashMap<String, ProductTargetConifg>>,

    /// # Passes
    /// Passes that can be purchased within your experience for Robux.
    pub passes: Option<HashMap<String, PassTargetConfig>>,

    /// # Badges
    /// Badges that can be awarded within your experience.
    pub badges: Option<HashMap<String, BadgeTargetConfig>>,

    /// # Assets
    /// A list of assets to include in your experience.
    pub assets: Option<Vec<AssetTargetConfig>>,
}

/// # SocialLink
/// ```yml title="Example"
/// target:
///   experience:
///     socialLinks:
///       - title: Follow on Twitter
///         url: https://twitter.com/blakemdev
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialLinkTargetConfig {
    /// # Title
    /// The display name of the social link on the Roblox website.
    pub title: String,
    /// # URL
    /// The URL of the social link. Must be one of the Roblox supported social link types.
    pub url: Url,
}

/// # Genre
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
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

/// # Playability
/// If set to `"public"`, the epxerience will be playable by all Roblox users.
///
/// If set to `"private"`, the experience will only be playable by the authorized user.
///
/// If set to `"friends"`, the experience will only be playable by the authorized user and that
/// user's Roblox friends.
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayabilityTargetConfig {
    Public,
    Private,
    Friends,
}

/// # AvatarType
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AvatarTypeTargetConfig {
    R6,
    R15,
    PlayerChoice,
}

/// # PlayableDevice
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayableDeviceTargetConfig {
    Computer,
    Phone,
    Tablet,
    Console,
}

/// # AnimationType
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AnimationTypeTargetConfig {
    Standard,
    PlayerChoice,
}

/// # CollisionType
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CollisionTypeTargetConfig {
    OuterBox,
    InnerBox,
}

/// # Constraint
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    /// # Minimum
    /// The minimum value (float)
    pub min: Option<f32>,
    /// # Maximum
    /// The maximum value (float)
    pub max: Option<f32>,
}

/// # AvatarScaleConstraints
/// ```yml title="Example"
/// target:
///   experience:
///     configuration:
///       avatarScaleConstraints:
///         height:
///           min: 0.95
///         width:
///           max: 0.9
///         proportions:
///           min: 30
///           max: 50
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AvatarScaleConstraintsTargetConfig {
    /// # Height
    /// The constraints to apply to the height of the avatar.
    pub height: Option<Constraint>,
    /// # Width
    /// The constraints to apply to the width of the avatar.
    pub width: Option<Constraint>,
    /// # Head
    /// The constraints to apply to the head of the avatar.
    pub head: Option<Constraint>,
    /// # Body Type
    /// The constraints to apply to the body type of the avatar.
    pub body_type: Option<Constraint>,
    /// # Proportions
    /// The constraints to apply to the proportions of the avatar.
    pub proportions: Option<Constraint>,
}

/// # AvatarAssetOverrides
/// ```yml title="Example"
/// target:
///   experience:
///     configuration:
///       avatarAssetOverrides:
///         face: 7699174
///         shirt: 5382048848
///         pants: 5611120855
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AvatarAssetOverridesTargetConfig {
    /// # Face
    /// The asset ID to override the avatar's face.
    pub face: Option<AssetId>,
    /// # Head
    /// The asset ID to override the avatar's head.
    pub head: Option<AssetId>,
    /// # Torso
    /// The asset ID to override the avatar's torso.
    pub torso: Option<AssetId>,
    /// # Left Arm
    /// The asset ID to override the avatar's left arm.
    pub left_arm: Option<AssetId>,
    /// # Right Arm
    /// The asset ID to override the avatar's right arm.
    pub right_arm: Option<AssetId>,
    /// # Left Leg
    /// The asset ID to override the avatar's left leg.
    pub left_leg: Option<AssetId>,
    /// # Right Leg
    /// The asset ID to override the avatar's right leg.
    pub right_leg: Option<AssetId>,
    #[serde(rename = "tshirt")]
    /// # T-shirt
    /// The asset ID to override the avatar's t-shirt.
    pub t_shirt: Option<AssetId>,
    /// # Shirt
    /// The asset ID to override the avatar's shirt.
    pub shirt: Option<AssetId>,
    /// # Pants
    /// The asset ID to override the avatar's pants.
    pub pants: Option<AssetId>,
}

/// # Product
/// ```yml title="Example"
/// target:
///   experience:
///     products:
///       fiftyGold:
///         name: 50 Gold
///         description: Add 50 gold to your wallet!
///         icon: products/50-gold.png
///         price: 25
///       hundredGold:
///         name: 100 Gold
///         description: Add 100 gold to your wallet!
///         icon: products/100-gold.png
///         price: 45
/// ```
///
/// Because Roblox does not offer any way to delete developer products, when a product is "deleted"
/// by Mantle, it is updated in the following ways:
///
/// 1. Its description is updated to: `Name: <name>\nDescription:\n<description>`
/// 2. Its name is updated to `zzz_Deprecated(<date-time>)` where `<date-time>` is the current
///    date-time in `YYYY-MM-DD hh::mm::ss.ns` format.
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductTargetConifg {
    /// # Name
    /// The display name of the developer product on the Roblox website and in the experience.
    pub name: String,
    /// # Description
    /// The description of the developer product on the Roblox website and in the experience.
    /// Defaults to `""`.
    pub description: Option<String>,
    /// # Icon
    /// A file path to an image to use as the product's icon on the Roblox website and in the
    /// experience. Defaults to no icon.
    pub icon: Option<String>,
    /// # Price
    /// The price of the developer product in Robux.
    pub price: u32,
}

/// # Pass
/// ```yml title="Example"
/// target:
///   experience:
///     passes:
///       shipOfTheLine:
///         name: Ship of the Line
///         description: Get the best ship in the game!
///         icon: passes/ship-of-the-line.png
///         price: 499
/// ```
///
/// Because Roblox does not offer any way to delete game passes, when a pass is "deleted" by
/// Mantle, it is updated in the following ways:
///
/// 1. Its description is updated to: `Name: <name>\nPrice: <price>\nDescription:\n<description>`
/// 2. Its name is updated to `zzz_Deprecated(<date-time>)` where `<date-time>` is the current date-time
///    in `YYYY-MM-DD hh::mm::ss.ns` format.
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassTargetConfig {
    /// # Name
    /// The display name of the game pass on the Roblox website and in the experience.
    pub name: String,
    /// # Description
    /// The description of the game pass on the Roblox website and in the experience. Defaults to
    /// `""`.
    pub description: Option<String>,
    /// # Icon
    /// A file path to an image to use as the pass's icon on the Roblox website and in the
    /// experience.
    pub icon: String,
    /// # Price
    /// The price of the game pass in Robux. If not specified, the game pass will be off-sale.
    pub price: Option<u32>,
}

/// # Badge
/// ```yml title="Example"
/// target:
///   experience:
///     badges:
///       captureFirstShip:
///         name: Capture First Ship!
///         description: You captured your first enemy ship!
///         icon: badges/capture-first-ship.png
/// ```
///
/// :::caution
/// By default, Mantle does not have permission to make purchases with Robux. Since creating badges
/// costs Robux, you will need to use the `--allow-purchases` flag when you want to create them.
/// :::
///
/// Because Roblox does not offer any way to delete badges, when a badge is "deleted" by
/// Mantle, it is updated in the following ways:
///
/// 1. It is disabled
/// 2. Its description is updated to: `Name: <name>\nEnabled: <enabled>\nDescription:\n<description>`
/// 3. Its name is updated to `zzz_Deprecated(<date-time>)` where `<date-time>` is the current date-time
///    in `YYYY-MM-DD hh::mm::ss.ns` format.
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeTargetConfig {
    /// # Name
    /// The display name of the badge on the Roblox website and in the experience.
    pub name: String,
    /// # Description
    /// The description of the badge on the Roblox website and in the experience. Defaults to `""`.
    pub description: Option<String>,
    /// # Icon
    /// A file path to an image to use as the badge's icon on the Roblox website and in the
    /// experience.
    pub icon: String,
    /// # Enabled
    /// Whether or not the badge is enabled. Defaults to `true`.
    pub enabled: Option<bool>,
}

/// # Asset
/// If set to `"file"`, the value should be a file path or glob to an asset or list of assets. The
/// `rbxgameasset` name of each matched file will be its file name without the extension. For
/// example, `assets/pirate-flag.png` will be given the `rbxgameasset` name `pirate-flag` and will
/// be accessible in the experience with `rbxgameasset://Images/pirate-flag`.
///
/// If set to an object, `file` should be set to a file path (it will not be interpreted as a glob),
/// and `name` will be the name of the `rbxgameasset`.
///
/// ```yml title="Example"
/// target:
///   experience:
///     assets:
///       - assets/*
///       - file: marketing/icon.png
///         name: game-icon
/// ```
///
/// :::caution
/// By default, Mantle does not have permission to make purchases with Robux. Since
/// creating and updating audio assets costs Robux, you will need to use the `--allow-purchases`
/// flag when you want to create or update them.
/// :::
///
/// Each file will be uploaded as the asset type matching its file extension. Supported asset types
/// and their file extensions:
///
/// | Asset type | File extensions                                 |
/// | :--------- | :---------------------------------------------- |
/// | Image      | `.bmp`, `.gif`, `.jpeg`, `.jpg`, `.png`, `.tga` |
/// | Audio      | `.ogg`, `.mp3`                                  |
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AssetTargetConfig {
    File(String),
    FileWithAlias { file: String, name: String },
}

/// # ExperienceConfiguration
/// ```yml title="Example"
/// target:
///   experience:
///     configuration:
///       genre: naval
///       playableDevices: [computer]
///       playability: private
///       privateServerPrice: 0
///       enableStudioAccessToApis: true
///       icon: marketing/game-icon.png
///       thumbnails:
///         - marketing/game-thumbnail-fall-update.png
///         - marketing/game-thumbnail-default.png
/// ```
///
/// In order to configure the name and description of an experience, use the `name` and
/// `description` fields of the `PlaceConfiguration` for the experience's start place.
#[derive(JsonSchema, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceTargetConfigurationConfig {
    // basic info
    /// # Genre
    /// The experience's genre. Defaults to `"all"`.
    pub genre: Option<GenreTargetConfig>,
    /// # Playable Devices
    /// The devices that the experience can be played on. Defaults to `["computer", "phone",
    /// "tablet"]`
    pub playable_devices: Option<Vec<PlayableDeviceTargetConfig>>,
    /// # Icon
    /// A file path to an image that will be used as the experience's icon.
    pub icon: Option<String>,
    /// # Thumbnails
    /// An array of file paths to images that will be used as the experience's thumbnails. The order
    /// used here will be the order they appear on the Roblox webpage.
    pub thumbnails: Option<Vec<String>>,

    // permissions
    /// # Playability
    /// Determines who has access to play the experience. Defaults to `"private"`.
    pub playability: Option<PlayabilityTargetConfig>,

    // monetization
    /// # Paid Access Price
    /// If set, paid access will be enabled with the specified price. Otherwise, paid access will be
    /// disabled. Should not be used with `privateServerPrice`.
    pub paid_access_price: Option<u32>,
    /// # Private Server Price
    /// If set, private servers will be enabled with the specified price. Otherwise, private servers
    /// will be disabled. To enable for free, set to `0`. Should not be used with
    /// `privateServerPrice`.
    pub private_server_price: Option<u32>,

    // security
    /// # Enable Studio Access to APIs
    /// Whether or not studio should be able to use Roblox APIs for this place. Defaults to `false`.
    pub enable_studio_access_to_apis: Option<bool>,
    /// # Allow Third-party Sales
    /// Whether or not this experience should allow third-party sales. Defaults to `false`.
    pub allow_third_party_sales: Option<bool>,
    /// # Allow Third-party Teleports
    /// Whether or not this experience should allow third-party teleports. Defaults to `false`.
    pub allow_third_party_teleports: Option<bool>,

    // localization: // TODO: localization

    // avatar
    /// # Avatar Type
    /// The types of avatars that players can use in this experience. Defaults to `"r15"`.
    pub avatar_type: Option<AvatarTypeTargetConfig>,
    /// # Avatar Animation Type
    /// The type of avatar animation that players can use in this experience. Defaults to
    /// `"playerChoice"`.
    pub avatar_animation_type: Option<AnimationTypeTargetConfig>,
    /// # Avatar Collision Type
    /// The type of avatar collision that players can use in this experience. Defaults to
    /// `"outerBox"`.
    pub avatar_collision_type: Option<CollisionTypeTargetConfig>,
    /// # Avatar Scale Constraints
    /// The scale constraints to apply to player avatars in the experience. Defaults to Roblox's
    /// defaults.
    pub avatar_scale_constraints: Option<AvatarScaleConstraintsTargetConfig>,
    /// # Avatar Asset Overrides
    /// The asset overrides to apply to player avatars in the experience. Defaults to Roblox's
    /// defaults.
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

/// # ServerFill
/// If set to `"robloxOptimized"`, Roblox will attempt to automatically leave some space for friends
/// to join.
///
/// If set to `"maximum"`, Roblox will never leave room for friends to join.
///
/// If set to reserved slots, Roblox will always leave the provided number of slots available for
/// friends to join.
///
/// ```yml title="Reserved Slots Example"
/// target:
///   experience:
///     places:
///       start:
///         file: game.rbxlx
///         configuration:
///           serverFill:
///             reservedSlots: 10
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServerFillTargetConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

/// # Place
/// ```yml title="Example"
/// target:
///   experience:
///     places:
///       start:
///         file: game.rbxlx
///         configuration:
///           name: Pirate Wars!
///           description: |-
///             Duke it out on the high seas in your pirate ship!
///
///             🍂 Fall update: new cannons, new ship types!
///           maxPlayerCount: 10
///           serverFill: robloxOptimized
/// ```
#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTargetConfig {
    /// # File
    /// A file path to a Roblox place (either `.rbxl` or `.rbxlx`).
    pub file: Option<String>,
    /// # Configuration
    /// A place's Roblox configuration.
    pub configuration: Option<PlaceTargetConfigurationConfig>,
}

/// # PlaceConfiguration
#[derive(JsonSchema, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTargetConfigurationConfig {
    /// # Name
    /// The display name of the place on the Roblox website and in-game. Defaults to `"Untitled
    /// Game"`. If the place is an experience's start place, it will be the experience's display
    /// name as well.
    pub name: Option<String>,
    /// # Description
    /// The descirption of the place on the Roblox website and in-game. Defaults to `"Created with
    /// Mantle"`. If the place is an experience's start place, it will be the experience's
    /// description as well.
    pub description: Option<String>,
    /// # Max Player Count
    /// The maximum number of players that can be in a server for the place. Defaults to `50`.
    pub max_player_count: Option<u32>,
    /// # Allow Copying
    /// Whether or not other Roblox users can clone your place. Defaults to `false`.
    pub allow_copying: Option<bool>,
    /// # Server Fill
    /// Determines how Roblox will fill your servers.
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
