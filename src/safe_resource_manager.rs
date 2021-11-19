use serde::{Deserialize, Serialize};

use crate::{
    roblox_api::{ExperienceConfigurationModel, PlaceConfigurationModel, SocialLinkType},
    safe_resources::{all_outputs, single_output, Resource, ResourceId, ResourceManager},
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
    file_path: String,
    file_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceInputs {
    is_start: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocialLinkInputs {
    title: String,
    url: String,
    link_type: SocialLinkType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperProductInputs {
    name: String,
    description: String,
    price: u32,
    icon_asset_id: Option<AssetId>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamePassInputs {
    name: String,
    description: Option<String>,
    price: Option<u32>,
    icon_file_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeInputs {
    name: String,
    description: Option<String>,
    enabled: bool,
    icon_file_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileWithGroupIdInputs {
    file_path: String,
    file_hash: String,
    group_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetAliasInputs {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImplInputs {
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
    DeveloperProduct(DeveloperProductInputs),
    DeveloperProductIcon(FileInputs),
    GamePass(GamePassInputs),
    GamePassIcon(FileInputs),
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
pub struct DeveloperProductOutputs {
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
pub enum ImplOutputs {
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
    DeveloperProduct(DeveloperProductOutputs),
    DeveloperProductIcon(AssetOutputs),
    GamePass(AssetWithInitialIconOutputs),
    GamePassIcon(AssetOutputs),
    Badge(AssetWithInitialIconOutputs),
    BadgeIcon(AssetOutputs),
    ImageAsset(ImageAssetOutputs),
    AudioAsset(AssetOutputs),
    AssetAlias(AssetAliasOutputs),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImplResource {
    id: ResourceId,
    inputs: ImplInputs,
    outputs: Option<ImplOutputs>,
    dependencies: Vec<ResourceId>,
}

impl Resource<ImplInputs, ImplOutputs> for ImplResource {
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

    fn get_inputs(&self) -> ImplInputs {
        self.inputs.clone()
    }

    fn get_outputs(&self) -> Option<ImplOutputs> {
        self.outputs.clone()
    }

    fn get_dependencies(&self) -> Vec<ResourceId> {
        self.dependencies.clone()
    }

    fn set_outputs(&mut self, outputs: ImplOutputs) {
        self.outputs = Some(outputs);
    }
}

struct ImplResourceManager {}

impl ResourceManager<ImplInputs, ImplOutputs> for ImplResourceManager {
    fn get_create_price(
        &mut self,
        inputs: ImplInputs,
        dependency_outputs: Vec<ImplOutputs>,
    ) -> Result<Option<u32>, String> {
        todo!()
    }

    fn create(
        &mut self,
        inputs: ImplInputs,
        dependency_outputs: Vec<ImplOutputs>,
    ) -> Result<ImplOutputs, String> {
        match inputs {
            ImplInputs::Experience(inputs) => {
                let experience_outputs =
                    single_output!(dependency_outputs, ImplOutputs::Experience);
                // let thumbnail_outputs = all_outputs!(dependency_outputs, ImplOutputs::Asset);

                // TODO: make requests and return outputs
            }
            _ => {}
        }
        unimplemented!()
    }

    fn get_update_price(
        &mut self,
        inputs: ImplInputs,
        outputs: ImplOutputs,
        dependency_outputs: Vec<ImplOutputs>,
    ) -> Result<Option<u32>, String> {
        todo!()
    }

    fn update(
        &mut self,
        inputs: ImplInputs,
        outputs: ImplOutputs,
        dependency_outputs: Vec<ImplOutputs>,
    ) -> Result<ImplOutputs, String> {
        todo!()
    }

    fn delete(
        &mut self,
        inputs: ImplInputs,
        outputs: ImplOutputs,
        dependency_outputs: Vec<ImplOutputs>,
    ) -> Result<(), String> {
        todo!()
    }
}
