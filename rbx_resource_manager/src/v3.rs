use rbx_api::models::{ExperienceConfigurationModel, PlaceConfigurationModel};
use serde::{Deserialize, Serialize};

use self::{inputs::*, outputs::*};

pub mod inputs {
    use rbx_api::models::{AssetId, SocialLinkType};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ExperienceInputs {
        pub group_id: Option<AssetId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ExperienceActivationInputs {
        pub is_active: bool,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FileInputs {
        pub file_path: String,
        pub file_hash: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PlaceInputs {
        pub is_start: bool,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SocialLinkInputs {
        pub title: String,
        pub url: String,
        pub link_type: SocialLinkType,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ProductInputs {
        pub name: String,
        pub description: String,
        pub price: u32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PassInputs {
        pub name: String,
        pub description: String,
        pub price: Option<u32>,
        pub icon_file_path: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct BadgeInputs {
        pub name: String,
        pub description: String,
        pub enabled: bool,
        pub icon_file_path: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FileWithGroupIdInputs {
        pub file_path: String,
        pub file_hash: String,
        pub group_id: Option<AssetId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AssetAliasInputs {
        pub name: String,
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum RobloxInputs {
    Experience(ExperienceInputs),
    ExperienceConfiguration(ExperienceConfigurationModel),
    ExperienceActivation(ExperienceActivationInputs),
    ExperienceIcon(FileInputs),
    ExperienceThumbnail(FileInputs),
    ExperienceThumbnailOrder,
    Place(PlaceInputs),
    PlaceFile(FileInputs),
    PlaceConfiguration(PlaceConfigurationModel),
    SocialLink(SocialLinkInputs),
    Product(ProductInputs),
    ProductIcon(FileInputs),
    Pass(PassInputs),
    PassIcon(FileInputs),
    Badge(BadgeInputs),
    BadgeIcon(FileInputs),
    ImageAsset(FileWithGroupIdInputs),
    AudioAsset(FileWithGroupIdInputs),
    AssetAlias(AssetAliasInputs),
}

pub mod outputs {
    use rbx_api::models::AssetId;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ExperienceOutputs {
        pub asset_id: AssetId,
        pub start_place_id: AssetId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AssetOutputs {
        pub asset_id: AssetId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PlaceFileOutputs {
        pub version: u32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ProductOutputs {
        pub asset_id: AssetId,
        pub product_id: AssetId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AssetWithInitialIconOutputs {
        pub asset_id: AssetId,
        pub initial_icon_asset_id: AssetId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageAssetOutputs {
        pub asset_id: AssetId,
        pub decal_asset_id: Option<AssetId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AssetAliasOutputs {
        pub name: String,
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RobloxOutputs {
    Experience(ExperienceOutputs),
    ExperienceConfiguration,
    ExperienceActivation,
    ExperienceIcon(AssetOutputs),
    ExperienceThumbnail(AssetOutputs),
    ExperienceThumbnailOrder,
    Place(AssetOutputs),
    PlaceFile(PlaceFileOutputs),
    PlaceConfiguration,
    SocialLink(AssetOutputs),
    Product(ProductOutputs),
    ProductIcon(AssetOutputs),
    Pass(AssetWithInitialIconOutputs),
    PassIcon(AssetOutputs),
    Badge(AssetWithInitialIconOutputs),
    BadgeIcon(AssetOutputs),
    ImageAsset(ImageAssetOutputs),
    AudioAsset(AssetOutputs),
    AssetAlias(AssetAliasOutputs),
}
