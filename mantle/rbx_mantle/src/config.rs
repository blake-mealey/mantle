use std::{
    collections::HashMap,
    fmt, fs,
    path::{Path, PathBuf},
    str,
};

use rbx_api::{
    experiences::models::{
        ExperienceAnimationType, ExperienceAvatarType, ExperienceCollisionType,
        ExperienceConfigurationModel, ExperienceGenre, ExperiencePlayableDevice,
    },
    models::{AssetId, AssetTypeId, SocialSlotType},
    places::models::PlaceConfigurationModel,
};
use rusoto_core::Region;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;
use yansi::Paint;

#[derive(JsonSchema, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    /// default('personal')
    /// skip_properties()
    ///
    /// The owner of the resources that will be created.
    ///
    /// | Value         | Description                                                     |
    /// |---------------|-----------------------------------------------------------------|
    /// | `'personal'`  | All resources will be created in the authorizer user's account. |
    /// | `group: <id>` | All resources will be created in the specified group's account. |
    ///
    /// ```yml title="Personal Example (Default)"
    /// owner: personal
    /// ```
    ///
    /// ```yml title="Group Example"
    /// owner:
    ///   group: 5723117
    /// ```
    #[serde(default)]
    pub owner: OwnerConfig,

    /// default('owner')
    ///
    /// Determines which account should make payments when creating resources
    /// that cost Robux. Note that Mantle will never make purchases unless the
    /// `--allow-purchases` flag is enabled.
    ///
    /// | Value        | Description                                                                                                                                                                                                                                                              |
    /// |--------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `'owner'`    | All payments will come from the balance of the user or group specified by the [`owner`](#owner) property.                                                                                                                                                                |
    /// | `'personal'` | All payments will come from the balance of the authorized user.                                                                                                                                                                                                          |
    /// | `'group'`    | All payments will come from the balance of the group specified by the [`owner`](#owner) property. Payments can only be set to `'group'` when the owner is also set to a group because Roblox does not currently allow groups to pay for resources outside of themselves. |
    #[serde(default)]
    pub payments: PaymentsConfig,

    /// The list of environments which Mantle can deploy to.
    ///
    /// ```yml title="Example"
    /// environments:
    ///   - label: staging
    ///     branches: [dev, dev/*]
    ///     targetOverrides:
    ///       configuration:
    ///         icon: marketing/beta-game-icon.png
    ///   - label: production
    ///     branches: [main]
    ///     targetAccess: public
    /// ```
    pub environments: Vec<EnvironmentConfig>,

    /// Defines the target resource which Mantle will deploy to. Currently
    /// Mantle only supports targeting Experiences, but in the future it will
    /// support other types like Plugins and Models.
    ///
    /// ```yml title="Example"
    /// target:
    ///   experience: {}
    /// ```
    pub target: TargetConfig,

    /// default('local')
    ///
    /// Defines how Mantle should manage state files (locally or remotely).
    ///
    /// | Value              | Description                                                                                                                                                                                                           |
    /// |--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `'local'`          | Mantle will save and load its state to and from a local `.mantle-state.yml` file.                                                                                                                                     |
    /// | `localKey: <key>`  | Mantle will save and load its state to and from a local file using the provided key with the format `<key>.mantle-state.yml`.                                                                                         |
    /// | `remote: <config>` | Mantle will save and load its state to and from a remote file stored in a cloud provider. Currently the only supported provider is Amazon S3. For more information, see the [Remote State](/docs/remote-state) guide. |
    ///
    /// ```yml title="Local State Example (Default)"
    /// state: local
    /// ```
    ///
    /// ```yml title="Custom Local State Example"
    /// state:
    ///   localKey: pirate-wars
    /// ```
    ///
    /// ```yml title="Remote State Example"
    /// state:
    ///   remote:
    ///     region: us-west-1
    ///     bucket: my-mantle-states
    ///     key: pirate-wars
    /// ```
    #[serde(default)]
    pub state: StateConfig,
}

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum OwnerConfig {
    #[default]
    Personal,
    Group(AssetId),
}

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum PaymentsConfig {
    #[default]
    Owner,
    Personal,
    Group,
}

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum StateConfig {
    #[default]
    Local,
    LocalKey(String),
    Remote(RemoteStateConfig),
}

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

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoteStateConfig {
    /// skip_properties()
    ///
    /// The AWS region your S3 bucket is located in. If for some reason you need
    /// to use a region that is not defined, you can supply a custom one:
    ///
    /// ```yml title="Custom Region Example"
    /// state:
    ///   remote:
    ///     region:
    ///       custom:
    ///         name: region-name
    ///         endpoint: region-endpoint
    ///     bucket: my-mantle-states
    ///     key: pirate-wars
    /// ```
    #[serde(with = "RegionRef")]
    pub region: Region,

    /// The name of your AWS S3 bucket.
    pub bucket: String,

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

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvironmentConfig {
    /// The label of the environment that is used to identify the environment
    /// via the `--environment` flag. Must be unique across all environments.
    pub label: String,

    /// An array of file globs to match against Git branches. If the
    /// `--environment` flag is not specified, Mantle will pick the first
    /// environment which contains a matching file glob for the current Git
    /// branch. If no environments match, Mantle will exit with a success code.
    #[serde(default)]
    pub branches: Vec<String>,

    /// Whether or not to tag the commit with place file versions after
    /// successful deployments. It is recommended to only enable this on your
    /// production environment. Tags will be of the format `<label>-v<version>`
    /// where `<label>` is the label of the place and `<version>` is the place's
    /// Roblox version.
    ///
    /// For example, a start place with Roblox version 23 would have the tag
    /// `start-v23`.
    #[serde(default)]
    pub tag_commit: bool,

    /// skip_properties()
    ///
    /// Adds a prefix to the target's name configuration. The implementation is dependent on the
    /// target's type. For Experience targets, all place names will be updated with the prefix.
    ///
    /// | Value                | Description                                                                                                                                                                                                                                                                                                                               |
    /// |----------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `'environmentLabel'` | The target name prefix will use the format `[<ENVIRONMENT>] ` where `<ENVIRONMENT>` is the value of the environment's [`label`](#environments--label) property in all caps. For example, if the environment's label was `'dev'` and the target's name was "Made with Mantle", the resulting target name will be "[DEV] Made with Mantle". |
    /// | `custom: <prefix>`   | The target name prefix will be the supplied value.                                                                                                                                                                                                                                                                                        |
    ///
    /// ```yml title="Environment Label Example"
    /// environments:
    ///   - label: dev
    ///     targetNamePrefix: environmentLabel
    ///   - label: prod
    /// ```
    ///
    /// ```yml title="Custom Example"
    /// environments:
    ///   - label: dev
    ///     targetNamePrefix:
    ///       custom: 'Prefix: '
    ///   - label: prod
    /// ```
    pub target_name_prefix: Option<TargetNamePrefixConfig>,

    /// Overrides the target's access. The implementation is dependent on the
    /// target's type. For Experience targets, the
    /// [`playability`](#target-experience-configuration-playability) property
    /// will be overridden.
    ///
    /// | Value       | Description                                                                               |
    /// |-------------|-------------------------------------------------------------------------------------------|
    /// | `'public'`  | The target will be accessible to all Roblox users.                                        |
    /// | `'private'` | The target will only be accessible to the authorized user.                                |
    /// | `'friends'` | The target will only be accessible to the authorized user and that user's Roblox friends. |
    pub target_access: Option<TargetAccessConfig>,

    // TODO: This could break future target types. It is implemented this way in order to support schemars
    /// skip_properties()
    ///
    /// Environment-specific overrides for the target resource definition. This
    /// will override all configuration, including changes made by the
    /// [`targetNamePrefix`](#environments--targetnameprefix) and
    /// [`targetAccess`](#environments--targetaccess) properties.
    ///
    /// Override the target configuration. Should match the type of the target
    /// configuration.
    pub target_overrides: Option<TargetOverridesConfig>,
}

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetNamePrefixConfig {
    EnvironmentLabel,
    Custom(String),
}

#[derive(JsonSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetAccessConfig {
    Public,
    Private,
    Friends,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetConfig {
    /// The target resource will be an Experience.
    Experience(ExperienceTargetConfig),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum TargetOverridesConfig {
    Experience(ExperienceTargetConfig),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExperienceTargetConfig {
    /// The Experience's Roblox configuration.
    ///
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
    /// :::tip
    /// In order to configure the name and description of an experience, use the
    /// [`name`](#target-experience-places-label-configuration-name) and
    /// [`description`](#target-experience-places-label-configuration-description)
    /// properties of the experience's start place
    /// :::
    pub configuration: Option<ExperienceTargetConfigurationConfig>,

    /// The experience's places. There must be at least one place supplied with
    /// the label `'start'`, which will be used as the start place for the
    /// experience.
    ///
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
    pub places: Option<HashMap<String, PlaceTargetConfig>>,

    /// A file path to an image that will be used as the experience's icon.
    pub icon: Option<String>,

    /// An array of file paths to images that will be used as the experience's thumbnails. The order
    /// used here will be the order they appear on the Roblox webpage.
    pub thumbnails: Option<Vec<String>>,

    /// A list of social links that will appear on the experience's webpage.
    ///
    /// ```yml title="Example"
    /// target:
    ///   experience:
    ///     socialLinks:
    ///       - title: Follow on Twitter
    ///         url: https://twitter.com/blakemdev
    /// ```
    pub social_links: Option<Vec<SocialLinkTargetConfig>>,

    /// Products that can be purchased within your experience for Robux.
    ///
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
    pub products: Option<HashMap<String, ProductTargetConifg>>,

    /// Passes that can be purchased within your experience for Robux.
    ///
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
    pub passes: Option<HashMap<String, PassTargetConfig>>,

    /// Badges that can be awarded within your experience.
    ///
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
    /// Each user can create up to 5 badges for free every day. After that, badges cost 100 Robux each. By
    /// default, Mantle does not have permission to make purchases with Robux, so if you go over your daily
    /// quota, you will need to use the `--allow-purchases` flag to create them.
    /// :::
    ///
    /// Because Roblox does not offer any way to delete badges, when a badge is "deleted" by Mantle, it is
    /// updated in the following ways:
    ///
    /// 1. It is disabled
    /// 2. Its description is updated to: `Name: <name>\nEnabled: <enabled>\nDescription:\n<description>`
    /// 3. Its name is updated to `zzz_Deprecated(<date-time>)` where `<date-time>` is the current date-time
    ///    in `YYYY-MM-DD hh::mm::ss.ns` format.
    pub badges: Option<HashMap<String, BadgeTargetConfig>>,

    /// skip_properties()
    ///
    /// A list of assets to include in your experience.
    ///
    /// If set to a string, the value should be a file path or glob to an asset
    /// or list of assets. The `rbxgameasset` name of each matched file will be
    /// its file name without the extension. For example,
    /// `assets/pirate-flag.png` will be given the `rbxgameasset` name
    /// `pirate-flag` and will be accessible in the experience with
    /// `rbxgameasset://Images/pirate-flag`.
    ///
    /// If set to an object, `file` should be set to a file path (it will not be
    /// interpreted as a glob), and `name` will be the name of the
    /// `rbxgameasset`.
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
    /// Roblox provides each user a monthly quota of audio uploads. Mantle will let you know each time it
    /// uploads an audio asset how many uploads you have left and when your quota will reset.
    /// :::
    ///
    /// Each file will be uploaded as the asset type matching its file
    /// extension. Supported asset types and their file extensions:
    ///
    /// | Asset type | File extensions                                 |
    /// | :--------- | :---------------------------------------------- |
    /// | Image      | `.bmp`, `.gif`, `.jpeg`, `.jpg`, `.png`, `.tga` |
    /// | Audio      | `.ogg`, `.mp3`                                  |
    pub assets: Option<Vec<AssetTargetConfig>>,

    /// Spatial voice configuration.
    pub spatial_voice: Option<SpatialVoiceTargetConfig>,

    /// Notification strings configuration.
    pub notifications: Option<Vec<NotificationTargetConfig>>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NotificationTargetConfig {
    /// The display name of the notification string on the Roblox website.
    pub name: String,

    /// The content of the notification string.
    /// Must include {experienceName} placeholder and may include {displayName} placeholder once.
    pub content: String,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SocialLinkTargetConfig {
    /// The display name of the social link on the Roblox website.
    pub title: String,

    /// The URL of the social link. Must be one of the Roblox supported social link types.
    pub url: Url,
}

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

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayabilityTargetConfig {
    Public,
    Private,
    Friends,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum PaidAccessTargetConfig {
    #[default]
    Disabled,
    Price(u32),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum PrivateServersTargetConfig {
    #[default]
    Disabled,
    Free,
    Price(u32),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AvatarTypeTargetConfig {
    R6,
    R15,
    PlayerChoice,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PlayableDeviceTargetConfig {
    Computer,
    Phone,
    Tablet,
    Console,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AnimationTypeTargetConfig {
    Standard,
    PlayerChoice,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CollisionTypeTargetConfig {
    OuterBox,
    InnerBox,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Constraint {
    /// The minimum value (float)
    pub min: Option<f32>,
    /// The maximum value (float)
    pub max: Option<f32>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AvatarScaleConstraintsTargetConfig {
    /// The constraints to apply to the height of the avatar.
    pub height: Option<Constraint>,

    /// The constraints to apply to the width of the avatar.
    pub width: Option<Constraint>,

    /// The constraints to apply to the head of the avatar.
    pub head: Option<Constraint>,

    /// The constraints to apply to the body type of the avatar.
    pub body_type: Option<Constraint>,

    /// The constraints to apply to the proportions of the avatar.
    pub proportions: Option<Constraint>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AvatarAssetOverridesTargetConfig {
    /// The asset ID to override the avatar's face.
    pub face: Option<AssetId>,

    /// The asset ID to override the avatar's head.
    pub head: Option<AssetId>,

    /// The asset ID to override the avatar's torso.
    pub torso: Option<AssetId>,

    /// The asset ID to override the avatar's left arm.
    pub left_arm: Option<AssetId>,

    /// The asset ID to override the avatar's right arm.
    pub right_arm: Option<AssetId>,

    /// The asset ID to override the avatar's left leg.
    pub left_leg: Option<AssetId>,

    /// The asset ID to override the avatar's right leg.
    pub right_leg: Option<AssetId>,

    /// The asset ID to override the avatar's t-shirt.
    #[serde(rename = "tshirt")]
    pub t_shirt: Option<AssetId>,

    /// The asset ID to override the avatar's shirt.
    pub shirt: Option<AssetId>,

    /// The asset ID to override the avatar's pants.
    pub pants: Option<AssetId>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProductTargetConifg {
    /// The display name of the developer product on the Roblox website and in the experience.
    pub name: String,

    /// default('')
    ///
    /// The description of the developer product on the Roblox website and in the experience.
    pub description: Option<String>,

    /// A file path to an image to use as the product's icon on the Roblox website and in the
    /// experience.
    pub icon: Option<String>,

    /// The price of the developer product in Robux.
    pub price: u32,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PassTargetConfig {
    /// The display name of the game pass on the Roblox website and in the experience.
    pub name: String,

    /// default('')
    ///
    /// The description of the game pass on the Roblox website and in the experience.
    pub description: Option<String>,

    /// A file path to an image to use as the pass's icon on the Roblox website and in the
    /// experience.
    pub icon: String,

    /// The price of the game pass in Robux. If not specified, the game pass will be off-sale.
    pub price: Option<u32>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BadgeTargetConfig {
    /// The display name of the badge on the Roblox website and in the experience.
    pub name: String,

    /// default('')
    ///
    /// The description of the badge on the Roblox website and in the experience.
    pub description: Option<String>,

    /// A file path to an image to use as the badge's icon on the Roblox website and in the
    /// experience.
    pub icon: String,

    /// default(true)
    ///
    /// Whether or not the badge is enabled.
    pub enabled: Option<bool>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AssetTargetConfig {
    File(String),
    FileWithAlias { file: String, name: String },
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpatialVoiceTargetConfig {
    /// Whether or not spatial voice is enabled for the experience.
    pub enabled: bool,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExperienceTargetConfigurationConfig {
    /// default('all')
    ///
    /// The experience's genre.
    pub genre: Option<GenreTargetConfig>,

    /// default(['computer', 'phone', 'tablet'])
    ///
    /// The devices that the experience can be played on.
    pub playable_devices: Option<Vec<PlayableDeviceTargetConfig>>,

    /// default('private')
    ///
    /// Determines who has access to play the experience.
    ///
    /// | Value       | Description                                                                                 |
    /// |-------------|---------------------------------------------------------------------------------------------|
    /// | `'public'`  | The experience will be playable by all Roblox users.                                        |
    /// | `'private'` | The experience will only be playable by the authorized user.                                |
    /// | `'friends'` | The experience will only be playable to the authorized user and that user's Roblox friends. |
    pub playability: Option<PlayabilityTargetConfig>,

    /// default('disabled')
    /// skip_properties()
    ///
    /// Determines whether or not paid access is be enabled, and if it is, how
    /// much it costs. This should not be enabled when
    /// [`privateServers`](#target-experience-configuration-privateservers) are
    /// also enabled as they are incompatible.
    ///
    /// | Value            | Description                                                                                      |
    /// |------------------|--------------------------------------------------------------------------------------------------|
    /// | `'disabled'`     | Paid access will be disabled.                                                                    |
    /// | `price: <price>` | Paid access will be enabled and will cost the provided number of Robux. Must be a minimum of 25. |
    ///
    /// ```yml title="Enabled Example"
    /// target:
    ///   experience:
    ///     configuration:
    ///       paidAccess:
    ///         price: 100
    /// ```
    #[serde(default)]
    pub paid_access: PaidAccessTargetConfig,

    /// default('disabled')
    /// skip_properties()
    ///
    /// Determines whether or not private servers are enabled, and if they are,
    /// how much they cost. This should not be enabled when
    /// [`paidAccess`](#target-experience-configuration-paidaccess) is also
    /// enabled as they are incompatible.
    ///
    /// | Value            | Description                                                                 |
    /// |------------------|-----------------------------------------------------------------------------|
    /// | `'disabled'`     | Private servers will be disabled.                                           |
    /// | `'free'`         | Private servers will be enabled and will be free to purchase.               |
    /// | `price: <price>` | Private servers will be enabled and will cost the provided number of Robux. |
    ///
    /// ```yml title="Enabled for Free Example"
    /// target:
    ///   experience:
    ///     configuration:
    ///       privateServers: free
    /// ```
    ///
    /// ```yml title="Enabled for Price Example"
    /// target:
    ///   experience:
    ///     configuration:
    ///       privateServers:
    ///         price: 100
    /// ```
    #[serde(default)]
    pub private_servers: PrivateServersTargetConfig,

    /// default(false)
    ///
    /// Whether or not studio should be able to use Roblox APIs for this place.
    pub enable_studio_access_to_apis: Option<bool>,

    /// default(false)
    ///
    /// Whether or not this experience should allow third-party sales.
    pub allow_third_party_sales: Option<bool>,

    /// default(false)
    ///
    /// Whether or not this experience should allow third-party teleports.
    pub allow_third_party_teleports: Option<bool>,

    /// default('r15')
    ///
    /// The types of avatars that players can use in this experience.
    pub avatar_type: Option<AvatarTypeTargetConfig>,

    /// default('playerChoice')
    ///
    /// The type of avatar animation that players can use in this experience.
    pub avatar_animation_type: Option<AnimationTypeTargetConfig>,

    /// default('outerBox')
    ///
    /// The type of avatar collision that players can use in this experience.
    pub avatar_collision_type: Option<CollisionTypeTargetConfig>,

    /// skip_properties()
    ///
    /// The scale constraints to apply to player avatars in the experience.
    /// Defaults to Roblox's defaults. Each entry may include a `min`, `max`, or
    /// both. If one is excluded, the default will be used.
    ///
    /// Supported properties: `bodyType`, `head`, `height`, `proportions`,
    /// `width`.
    ///
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
    pub avatar_scale_constraints: Option<AvatarScaleConstraintsTargetConfig>,

    /// skip_properties()
    ///
    /// The asset overrides to apply to player avatars in the experience.
    /// Defaults to Roblox's defaults.
    ///
    /// Supported properties: `face`, `head`, `leftArm`, `leftLeg`, `rightArm`,
    /// `rightLeg`, `torso`, `tshirt`, `shirt`, `pants`
    ///
    /// ```yml title="Example"
    /// target:
    ///   experience:
    ///     configuration:
    ///       avatarAssetOverrides:
    ///         face: 7699174
    ///         shirt: 5382048848
    ///         pants: 5611120855
    /// ```
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
        model.is_for_sale = !matches!(config.paid_access, PaidAccessTargetConfig::Disabled);
        model.price = match config.paid_access {
            PaidAccessTargetConfig::Price(price) => Some(price),
            _ => None,
        };
        model.allow_private_servers =
            !matches!(config.private_servers, PrivateServersTargetConfig::Disabled);
        model.private_server_price = match config.private_servers {
            PrivateServersTargetConfig::Free => Some(0),
            PrivateServersTargetConfig::Price(price) => Some(price),
            _ => None,
        };
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

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ServerFillTargetConfig {
    RobloxOptimized,
    Maximum,
    ReservedSlots(u32),
}

#[derive(JsonSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlaceTargetConfig {
    /// A file path to a Roblox place (either `.rbxl` or `.rbxlx`).
    pub file: Option<String>,

    /// A place's Roblox configuration.
    pub configuration: Option<PlaceTargetConfigurationConfig>,
}

#[derive(JsonSchema, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlaceTargetConfigurationConfig {
    /// default('Untitled Game')
    ///
    /// The display name of the place on the Roblox website and in-game. If the
    /// place is an experience's start place, it will be the experience's
    /// display name as well.
    pub name: Option<String>,

    /// default('Created with Mantle')
    ///
    /// The descirption of the place on the Roblox website and in-game. If the
    /// place is an experience's start place, it will be the experience's
    /// description as well.
    pub description: Option<String>,

    /// default(50)
    ///
    /// The maximum number of players that can be in a server for the place.
    pub max_player_count: Option<u32>,

    /// default(false)
    ///
    /// Whether or not other Roblox users can clone your place.
    pub allow_copying: Option<bool>,

    /// default('robloxOptimized')
    /// skip_properties()
    ///
    /// Determines how Roblox will fill your servers.
    ///
    /// | Value                    | Description                                                                          |
    /// |--------------------------|--------------------------------------------------------------------------------------|
    /// | `'robloxOptimized'`      | Roblox will attempt to automatically leave some space for friends to join.           |
    /// | `'maximum'`              | Roblox will never leave room for friends to join.                                    |
    /// | `reservedSlots: <count>` | Roblox will always leave the provided number of slots available for friends to join. |
    ///
    /// ```yml title="Maximum Example"
    /// target:
    ///   experience:
    ///     places:
    ///       start:
    ///         file: game.rbxlx
    ///         configuration:
    ///           serverFill: maximum
    /// ```
    ///
    /// ```yml title="Reserved Slots Example"
    /// target:
    ///   experience:
    ///     places:
    ///       start:
    ///         file: game.rbxlx
    ///         configuration:
    ///           serverFill:
    ///             reservedSlots: 5
    /// ```
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
