use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    resource_graph::{
        all_outputs, optional_output, single_output, Resource, ResourceId, ResourceManager,
    },
    roblox_api::{
        CreateAudioAssetResponse, CreateBadgeResponse, CreateDeveloperProductResponse,
        CreateExperienceResponse, CreateGamePassResponse, CreateImageAssetResponse,
        CreateSocialLinkResponse, CreatorType, ExperienceConfigurationModel,
        GetCreateAudioAssetPriceResponse, GetDeveloperProductResponse, GetPlaceResponse,
        PlaceConfigurationModel, RobloxApi, SocialLinkType, UploadImageResponse,
    },
    roblox_auth::RobloxAuth,
};

pub type AssetId = u64;

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
    pub description: Option<String>,
    pub price: u32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassInputs {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<u32>,
    pub icon_file_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeInputs {
    pub name: String,
    pub description: Option<String>,
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RobloxResource {
    id: ResourceId,
    inputs: RobloxInputs,
    outputs: Option<RobloxOutputs>,
    dependencies: Vec<ResourceId>,
}

impl RobloxResource {
    pub fn new(id: &str, inputs: RobloxInputs, dependencies: &[&RobloxResource]) -> Self {
        Self {
            id: id.to_owned(),
            inputs,
            outputs: None,
            dependencies: dependencies.iter().map(|d| d.get_id()).collect(),
        }
    }

    pub fn existing(
        id: &str,
        inputs: RobloxInputs,
        outputs: RobloxOutputs,
        dependencies: &[&RobloxResource],
    ) -> Self {
        Self {
            id: id.to_owned(),
            inputs,
            outputs: Some(outputs),
            dependencies: dependencies.iter().map(|d| d.get_id()).collect(),
        }
    }

    pub fn add_dependency(&mut self, dependency: &RobloxResource) -> &mut Self {
        self.dependencies.push(dependency.get_id());
        self
    }
}

impl Resource<RobloxInputs, RobloxOutputs> for RobloxResource {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_inputs_hash(&self) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&self.inputs)
            .map_err(|e| format!("Failed to compute inputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }

    fn get_outputs_hash(&self) -> String {
        // TODO: Should we separate hashes from displays?
        let hash = serde_yaml::to_string(&self.outputs)
            .map_err(|e| format!("Failed to compute outputs hash\n\t{}", e))
            .unwrap();
        if hash.is_empty() {
            ""
        } else {
            // We remove first 4 characters to remove "---\n", and we trim the end to remove "\n"
            hash[4..].trim_end()
        }
        .to_owned()
    }

    fn get_inputs(&self) -> RobloxInputs {
        self.inputs.clone()
    }

    fn get_outputs(&self) -> Option<RobloxOutputs> {
        self.outputs.clone()
    }

    fn get_dependencies(&self) -> Vec<ResourceId> {
        self.dependencies.clone()
    }

    fn set_outputs(&mut self, outputs: RobloxOutputs) {
        self.outputs = Some(outputs);
    }
}

pub struct RobloxResourceManager {
    roblox_api: RobloxApi,
    project_path: PathBuf,
    payment_source: CreatorType,
}

impl RobloxResourceManager {
    pub fn new(project_path: &Path, payment_source: CreatorType) -> Self {
        Self {
            roblox_api: RobloxApi::new(RobloxAuth::new()),
            project_path: project_path.to_path_buf(),
            payment_source,
        }
    }

    fn get_path(&self, file: String) -> PathBuf {
        self.project_path.join(file)
    }
}

impl ResourceManager<RobloxInputs, RobloxOutputs> for RobloxResourceManager {
    fn get_create_price(
        &mut self,
        inputs: RobloxInputs,
        _dependency_outputs: Vec<RobloxOutputs>,
    ) -> Result<Option<u32>, String> {
        match inputs {
            RobloxInputs::Badge(_) => Ok(Some(100)),
            RobloxInputs::AudioAsset(inputs) => {
                let GetCreateAudioAssetPriceResponse {
                    price, can_afford, ..
                } = self.roblox_api.get_create_audio_asset_price(
                    self.get_path(inputs.file_path),
                    inputs.group_id,
                )?;

                // TODO: Add support for failing early like this for all other resource types (e.g. return the price and current balance from this function)
                if !can_afford {
                    return Err(format!("You do not have enough Robux to create an audio asset with the price of {}", price));
                }

                Ok(Some(price))
            }
            _ => Ok(None),
        }
    }

    fn create(
        &mut self,
        inputs: RobloxInputs,
        dependency_outputs: Vec<RobloxOutputs>,
    ) -> Result<RobloxOutputs, String> {
        match inputs {
            RobloxInputs::Experience(inputs) => {
                let CreateExperienceResponse {
                    universe_id,
                    root_place_id,
                } = self.roblox_api.create_experience(inputs.group_id)?;

                Ok(RobloxOutputs::Experience(ExperienceOutputs {
                    asset_id: universe_id,
                    start_place_id: root_place_id,
                }))
            }
            RobloxInputs::ExperienceConfiguration(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .configure_experience(experience.asset_id, &inputs)?;

                Ok(RobloxOutputs::ExperienceConfiguration)
            }
            RobloxInputs::ExperienceActivation(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .set_experience_active(experience.asset_id, inputs.is_active)?;

                Ok(RobloxOutputs::ExperienceActivation)
            }
            RobloxInputs::ExperienceIcon(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let UploadImageResponse { target_id } = self
                    .roblox_api
                    .upload_icon(experience.asset_id, self.get_path(inputs.file_path))?;

                Ok(RobloxOutputs::ExperienceIcon(AssetOutputs {
                    asset_id: target_id,
                }))
            }
            RobloxInputs::ExperienceThumbnail(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let UploadImageResponse { target_id } = self
                    .roblox_api
                    .upload_thumbnail(experience.asset_id, self.get_path(inputs.file_path))?;

                Ok(RobloxOutputs::ExperienceThumbnail(AssetOutputs {
                    asset_id: target_id,
                }))
            }
            RobloxInputs::ExperienceThumbnailOrder => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);
                let thumbnails =
                    all_outputs!(dependency_outputs, RobloxOutputs::ExperienceThumbnail);

                self.roblox_api.set_experience_thumbnail_order(
                    experience.asset_id,
                    &thumbnails.iter().map(|t| t.asset_id).collect::<Vec<_>>(),
                )?;

                Ok(RobloxOutputs::ExperienceThumbnailOrder)
            }
            RobloxInputs::Place(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let asset_id = if inputs.is_start {
                    experience.start_place_id
                } else {
                    self.roblox_api.create_place(experience.asset_id)?.place_id
                };

                Ok(RobloxOutputs::Place(AssetOutputs { asset_id }))
            }
            RobloxInputs::PlaceFile(inputs) => {
                let place = single_output!(dependency_outputs, RobloxOutputs::Place);

                self.roblox_api
                    .upload_place(self.get_path(inputs.file_path), place.asset_id)?;
                let GetPlaceResponse {
                    current_saved_version,
                    ..
                } = self.roblox_api.get_place(place.asset_id)?;

                Ok(RobloxOutputs::PlaceFile(PlaceFileOutputs {
                    version: current_saved_version,
                }))
            }
            RobloxInputs::PlaceConfiguration(inputs) => {
                let place = single_output!(dependency_outputs, RobloxOutputs::Place);

                self.roblox_api.configure_place(place.asset_id, &inputs)?;

                Ok(RobloxOutputs::PlaceConfiguration)
            }
            RobloxInputs::SocialLink(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let CreateSocialLinkResponse { id } = self.roblox_api.create_social_link(
                    experience.asset_id,
                    inputs.title,
                    inputs.url,
                    inputs.link_type,
                )?;

                Ok(RobloxOutputs::SocialLink(AssetOutputs { asset_id: id }))
            }
            RobloxInputs::ProductIcon(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let asset_id = self.roblox_api.create_developer_product_icon(
                    experience.asset_id,
                    self.get_path(inputs.file_path),
                )?;

                Ok(RobloxOutputs::ProductIcon(AssetOutputs { asset_id }))
            }
            RobloxInputs::Product(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);
                let icon = optional_output!(dependency_outputs, RobloxOutputs::ProductIcon);

                let CreateDeveloperProductResponse { id } =
                    self.roblox_api.create_developer_product(
                        experience.asset_id,
                        inputs.name,
                        inputs.price,
                        inputs.description,
                        icon.map(|i| i.asset_id),
                    )?;

                let GetDeveloperProductResponse { product_id, .. } = self
                    .roblox_api
                    .find_developer_product_by_id(experience.asset_id, id)?;

                Ok(RobloxOutputs::Product(ProductOutputs {
                    asset_id: product_id,
                    product_id: id,
                }))
            }
            RobloxInputs::Pass(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let CreateGamePassResponse {
                    asset_id,
                    icon_asset_id,
                } = self.roblox_api.create_game_pass(
                    experience.start_place_id,
                    inputs.name.clone(),
                    inputs.description.clone(),
                    self.get_path(inputs.icon_file_path),
                )?;
                self.roblox_api.update_game_pass(
                    asset_id,
                    inputs.name,
                    inputs.description,
                    inputs.price,
                )?;

                Ok(RobloxOutputs::Pass(AssetWithInitialIconOutputs {
                    asset_id,
                    initial_icon_asset_id: icon_asset_id,
                }))
            }
            RobloxInputs::PassIcon(_) => {
                let game_pass = single_output!(dependency_outputs, RobloxOutputs::Pass);

                Ok(RobloxOutputs::PassIcon(AssetOutputs {
                    asset_id: game_pass.initial_icon_asset_id,
                }))
            }
            RobloxInputs::Badge(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let CreateBadgeResponse { id, icon_image_id } = self.roblox_api.create_badge(
                    experience.asset_id,
                    inputs.name,
                    inputs.description,
                    self.get_path(inputs.icon_file_path),
                    self.payment_source.clone(),
                )?;

                Ok(RobloxOutputs::Badge(AssetWithInitialIconOutputs {
                    asset_id: id,
                    initial_icon_asset_id: icon_image_id,
                }))
            }
            RobloxInputs::BadgeIcon(_) => {
                let badge = single_output!(dependency_outputs, RobloxOutputs::Badge);

                Ok(RobloxOutputs::BadgeIcon(AssetOutputs {
                    asset_id: badge.initial_icon_asset_id,
                }))
            }
            RobloxInputs::ImageAsset(inputs) => {
                let CreateImageAssetResponse {
                    asset_id,
                    backing_asset_id,
                    ..
                } = self
                    .roblox_api
                    .create_image_asset(self.get_path(inputs.file_path), inputs.group_id)?;

                Ok(RobloxOutputs::ImageAsset(ImageAssetOutputs {
                    asset_id: backing_asset_id,
                    decal_asset_id: Some(asset_id),
                }))
            }
            RobloxInputs::AudioAsset(inputs) => {
                let CreateAudioAssetResponse { id } = self.roblox_api.create_audio_asset(
                    self.get_path(inputs.file_path),
                    inputs.group_id,
                    self.payment_source.clone(),
                )?;

                Ok(RobloxOutputs::AudioAsset(AssetOutputs { asset_id: id }))
            }
            RobloxInputs::AssetAlias(inputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let image_asset = optional_output!(dependency_outputs, RobloxOutputs::ImageAsset);
                let audio_asset = optional_output!(dependency_outputs, RobloxOutputs::AudioAsset);
                let asset_id = match (image_asset, audio_asset) {
                    (Some(image_asset), None) => image_asset.asset_id,
                    (None, Some(audio_asset)) => audio_asset.asset_id,
                    _ => panic!("Missing expected output."),
                };

                self.roblox_api.create_asset_alias(
                    experience.asset_id,
                    asset_id,
                    inputs.name.clone(),
                )?;

                Ok(RobloxOutputs::AssetAlias(AssetAliasOutputs {
                    name: inputs.name,
                }))
            }
        }
    }

    fn get_update_price(
        &mut self,
        inputs: RobloxInputs,
        outputs: RobloxOutputs,
        dependency_outputs: Vec<RobloxOutputs>,
    ) -> Result<Option<u32>, String> {
        match (inputs.clone(), outputs) {
            (RobloxInputs::AudioAsset(_), RobloxOutputs::AudioAsset(_)) => {
                self.get_create_price(inputs, dependency_outputs)
            }
            _ => Ok(None),
        }
    }

    // TODO: Consider moving `outputs` into `dependency_outputs`.
    fn update(
        &mut self,
        inputs: RobloxInputs,
        outputs: RobloxOutputs,
        dependency_outputs: Vec<RobloxOutputs>,
    ) -> Result<RobloxOutputs, String> {
        match (inputs.clone(), outputs.clone()) {
            (RobloxInputs::Experience(_), RobloxOutputs::Experience(_)) => {
                self.delete(outputs, dependency_outputs.clone())?;
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::ExperienceConfiguration(_), RobloxOutputs::ExperienceConfiguration) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::ExperienceActivation(_), RobloxOutputs::ExperienceActivation) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::ExperienceIcon(_), RobloxOutputs::ExperienceIcon(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::ExperienceThumbnail(_), RobloxOutputs::ExperienceThumbnail(_)) => {
                self.delete(outputs, dependency_outputs.clone())?;
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::ExperienceThumbnailOrder, RobloxOutputs::ExperienceThumbnailOrder) => {
                self.create(inputs, dependency_outputs)
            }
            // TODO: is this correct?
            (RobloxInputs::Place(_), RobloxOutputs::Place(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::PlaceFile(_), RobloxOutputs::PlaceFile(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::PlaceConfiguration(_), RobloxOutputs::PlaceConfiguration) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::SocialLink(inputs), RobloxOutputs::SocialLink(outputs)) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api.update_social_link(
                    experience.asset_id,
                    outputs.asset_id,
                    inputs.title,
                    inputs.url,
                    inputs.link_type,
                )?;

                Ok(RobloxOutputs::SocialLink(outputs))
            }
            (RobloxInputs::ProductIcon(_), RobloxOutputs::ProductIcon(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::Product(inputs), RobloxOutputs::Product(outputs)) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);
                let icon = optional_output!(dependency_outputs, RobloxOutputs::ProductIcon);

                self.roblox_api.update_developer_product(
                    experience.asset_id,
                    outputs.asset_id,
                    inputs.name,
                    inputs.price,
                    inputs.description,
                    icon.map(|i| i.asset_id),
                )?;

                Ok(RobloxOutputs::Product(outputs))
            }
            (RobloxInputs::Pass(inputs), RobloxOutputs::Pass(outputs)) => {
                self.roblox_api.update_game_pass(
                    outputs.asset_id,
                    inputs.name,
                    inputs.description,
                    inputs.price,
                )?;

                Ok(RobloxOutputs::Pass(outputs))
            }
            (RobloxInputs::PassIcon(inputs), RobloxOutputs::PassIcon(_)) => {
                let game_pass = single_output!(dependency_outputs, RobloxOutputs::Pass);

                let UploadImageResponse { target_id } = self
                    .roblox_api
                    .update_game_pass_icon(game_pass.asset_id, self.get_path(inputs.file_path))?;

                Ok(RobloxOutputs::PassIcon(AssetOutputs {
                    asset_id: target_id,
                }))
            }
            (RobloxInputs::Badge(inputs), RobloxOutputs::Badge(outputs)) => {
                self.roblox_api.update_badge(
                    outputs.asset_id,
                    inputs.name,
                    inputs.description,
                    inputs.enabled,
                )?;

                Ok(RobloxOutputs::Badge(outputs))
            }
            (RobloxInputs::BadgeIcon(inputs), RobloxOutputs::BadgeIcon(_)) => {
                let badge = single_output!(dependency_outputs, RobloxOutputs::Badge);

                let UploadImageResponse { target_id } = self
                    .roblox_api
                    .update_badge_icon(badge.asset_id, self.get_path(inputs.file_path))?;

                Ok(RobloxOutputs::BadgeIcon(AssetOutputs {
                    asset_id: target_id,
                }))
            }
            (RobloxInputs::ImageAsset(_), RobloxOutputs::ImageAsset(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::AudioAsset(_), RobloxOutputs::AudioAsset(_)) => {
                self.create(inputs, dependency_outputs)
            }
            (RobloxInputs::AssetAlias(inputs), RobloxOutputs::AssetAlias(outputs)) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let image_asset = optional_output!(dependency_outputs, RobloxOutputs::ImageAsset);
                let audio_asset = optional_output!(dependency_outputs, RobloxOutputs::AudioAsset);
                let asset_id = match (image_asset, audio_asset) {
                    (Some(image_asset), None) => image_asset.asset_id,
                    (None, Some(audio_asset)) => audio_asset.asset_id,
                    _ => panic!("Missing expected output."),
                };

                self.roblox_api.update_asset_alias(
                    experience.asset_id,
                    asset_id,
                    outputs.name,
                    inputs.name.clone(),
                )?;

                Ok(RobloxOutputs::AssetAlias(AssetAliasOutputs {
                    name: inputs.name,
                }))
            }
            _ => unreachable!(),
        }
    }

    // TODO: Do we need inputs?
    fn delete(
        &mut self,
        outputs: RobloxOutputs,
        dependency_outputs: Vec<RobloxOutputs>,
    ) -> Result<(), String> {
        match outputs {
            RobloxOutputs::Experience(outputs) => {
                let model = ExperienceConfigurationModel {
                    is_archived: true,
                    ..Default::default()
                };
                self.roblox_api
                    .configure_experience(outputs.asset_id, &model)?;
            }
            RobloxOutputs::ExperienceConfiguration => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let model = ExperienceConfigurationModel::default();
                self.roblox_api
                    .configure_experience(experience.asset_id, &model)?;
            }
            RobloxOutputs::ExperienceActivation => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .set_experience_active(experience.asset_id, false)?;
            }
            RobloxOutputs::ExperienceIcon(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .remove_experience_icon(experience.start_place_id, outputs.asset_id)?;
            }
            RobloxOutputs::ExperienceThumbnail(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .delete_experience_thumbnail(experience.asset_id, outputs.asset_id)?;
            }
            RobloxOutputs::ExperienceThumbnailOrder => {}
            RobloxOutputs::Place(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                if outputs.asset_id != experience.start_place_id {
                    self.roblox_api
                        .remove_place_from_experience(experience.asset_id, outputs.asset_id)?;
                }
            }
            RobloxOutputs::PlaceFile(_) => {}
            RobloxOutputs::PlaceConfiguration => {
                let place = single_output!(dependency_outputs, RobloxOutputs::Place);

                let model = PlaceConfigurationModel::default();
                self.roblox_api.configure_place(place.asset_id, &model)?;
            }
            RobloxOutputs::SocialLink(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .delete_social_link(experience.asset_id, outputs.asset_id)?;
            }
            RobloxOutputs::ProductIcon(_) => {}
            RobloxOutputs::Product(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                let utc = Utc::now();
                self.roblox_api.update_developer_product(
                    experience.asset_id,
                    outputs.asset_id,
                    format!("zzz_DEPRECATED({})", utc.format("%F %T%.f")),
                    0,
                    Some("".to_owned()),
                    None,
                )?;
            }
            RobloxOutputs::Pass(outputs) => {
                let utc = Utc::now();
                self.roblox_api.update_game_pass(
                    outputs.asset_id,
                    format!("zzz_DEPRECATED({})", utc.format("%F %T%.f")),
                    Some("".to_owned()),
                    None,
                )?;
            }
            RobloxOutputs::PassIcon(_) => {}
            RobloxOutputs::Badge(outputs) => {
                let utc = Utc::now();
                self.roblox_api.update_badge(
                    outputs.asset_id,
                    format!("zzz_DEPRECATED({})", utc.format("%F %T%.f")),
                    Some("".to_owned()),
                    false,
                )?;
            }
            RobloxOutputs::BadgeIcon(_) => {}
            RobloxOutputs::ImageAsset(outputs) => {
                // TODO: Can we make this not optional and just not import the image asset? Maybe?
                if let Some(decal_asset_id) = outputs.decal_asset_id {
                    self.roblox_api.archive_asset(decal_asset_id)?;
                }
            }
            RobloxOutputs::AudioAsset(outputs) => {
                self.roblox_api.archive_asset(outputs.asset_id)?;
            }
            RobloxOutputs::AssetAlias(outputs) => {
                let experience = single_output!(dependency_outputs, RobloxOutputs::Experience);

                self.roblox_api
                    .delete_asset_alias(experience.asset_id, outputs.name)?;
            }
        }
        Ok(())
    }
}
