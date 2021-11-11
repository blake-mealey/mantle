use multipart::client::lazy::{Multipart, PreparedFields};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{clone::Clone, collections::HashMap, default, ffi::OsStr, fmt, fs, path::Path};

use crate::{
    resource_manager::AssetId,
    roblox_auth::{AuthType, RequestExt, RobloxAuth},
};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DeployMode {
    Publish,
    Save,
}
impl default::Default for DeployMode {
    fn default() -> Self {
        DeployMode::Publish
    }
}

impl fmt::Display for DeployMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            DeployMode::Publish => "Publish",
            DeployMode::Save => "Save",
        };
        write!(f, "{}", display)
    }
}

#[derive(Deserialize, Debug)]
struct RobloxApiErrorModel {
    // There are some other possible properties but we currently have no use for them so they are not
    // included

    // Most error models have a `message` property
    message: Option<String>,

    // Some error models (500) have a `title` property instead
    title: Option<String>,

    // Some error models on older APIs have an errors array
    errors: Option<Vec<RobloxApiErrorModel>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateExperienceResponse {
    pub universe_id: AssetId,
    pub root_place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExperienceResponse {
    pub root_place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreatePlaceResponse {
    pub success: bool,
    pub place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaceResponse {
    pub current_saved_version: u32,
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
    pub shop_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeveloperProductsResponse {
    pub developer_products: Vec<GetDeveloperProductResponse>,
    pub final_page: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetDeveloperProductResponse {
    pub product_id: AssetId,
    pub developer_product_id: AssetId,
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

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub enum ExperienceAnimationType {
    Standard,
    PlayerChoice,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ExperienceCollisionType {
    OuterBox,
    InnerBox,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExperiencePermissionsModel {
    pub is_third_party_purchase_allowed: Option<bool>,
    pub is_third_party_teleport_allowed: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceConfigurationModel {
    pub genre: Option<ExperienceGenre>,
    pub playable_devices: Option<Vec<ExperiencePlayableDevice>>,
    pub is_friends_only: Option<bool>,

    pub allow_private_servers: Option<bool>,
    pub private_server_price: Option<u32>,
    pub is_for_sale: Option<bool>,
    pub price: Option<u32>,

    pub studio_access_to_apis_allowed: Option<bool>,
    pub permissions: Option<ExperiencePermissionsModel>,

    pub universe_avatar_type: Option<ExperienceAvatarType>,
    pub universe_animation_type: Option<ExperienceAnimationType>,
    pub universe_collision_type: Option<ExperienceCollisionType>,
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
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_player_count: Option<u32>,
    pub allow_copying: Option<bool>,
    pub social_slot_type: Option<SocialSlotType>,
    pub custom_social_slot_count: Option<u32>,
}

pub struct RobloxApi {
    roblox_auth: RobloxAuth,
}

impl RobloxApi {
    pub fn new(roblox_auth: RobloxAuth) -> Self {
        Self { roblox_auth }
    }

    fn get_roblox_api_error_message(response: ureq::Response) -> Option<String> {
        fn get_message_from_error(error: RobloxApiErrorModel) -> Option<String> {
            if let Some(message) = error.message {
                Some(message)
            } else if let Some(title) = error.title {
                Some(title)
            } else if let Some(errors) = error.errors {
                for e in errors {
                    if let Some(message) = get_message_from_error(e) {
                        return Some(message);
                    }
                }
                None
            } else {
                None
            }
        }

        match response.content_type() {
            "application/json" => match response.into_json::<RobloxApiErrorModel>() {
                Ok(v) => get_message_from_error(v),
                Err(_) => None,
            },
            "text/html" => {
                // println!("{}", response.into_string().unwrap());
                None
            }
            _ => response.into_string().ok(),
        }
    }

    fn handle_response(
        result: Result<ureq::Response, ureq::Error>,
    ) -> Result<ureq::Response, String> {
        match result {
            Ok(response) => Ok(response),
            Err(ureq::Error::Status(status, response)) => {
                match Self::get_roblox_api_error_message(response) {
                    Some(message) => Err(message),
                    None => Err(format!("Unknown error (status {})", status)),
                }
            }
            Err(e) => Err(format!("Unknown error: {}", e)),
        }
    }

    pub fn upload_place(&mut self, place_file: &Path, place_id: AssetId) -> Result<(), String> {
        let data = match fs::read_to_string(place_file) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "Unable to read place file: {}\n\t{}",
                    place_file.display(),
                    e
                ))
            }
        };

        let res = ureq::post("https://data.roblox.com/Data/Upload.ashx")
            .query("assetId", &place_id.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .set("Content-Type", "application/xml")
            .set("User-Agent", "Roblox/WinInet")
            .send_string(&data);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn get_place(&mut self, place_id: AssetId) -> Result<GetPlaceResponse, String> {
        let res = ureq::get(&format!(
            "https://develop.roblox.com/v2/places/{}",
            place_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetPlaceResponse>()
            .map_err(|e| format!("Failed to deserialize get place response: {}", e))?;

        Ok(model)
    }

    pub fn create_experience(&mut self) -> Result<CreateExperienceResponse, String> {
        let res = ureq::post("https://api.roblox.com/universes/create")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "templatePlaceIdToUse": 95206881
            }));

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreateExperienceResponse>()
            .map_err(|e| format!("Failed to deserialize create experience response: {}", e))?;

        Ok(model)
    }

    pub fn get_experience(
        &mut self,
        experience_id: AssetId,
    ) -> Result<GetExperienceResponse, String> {
        let res = ureq::get(&format!(
            "https://develop.roblox.com/v1/universes/{}",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetExperienceResponse>()
            .map_err(|e| format!("Failed to deserialize get experience response: {}", e))?;

        Ok(model)
    }

    pub fn create_place(&mut self, experience_id: AssetId) -> Result<CreatePlaceResponse, String> {
        let res = ureq::post("https://www.roblox.com/ide/places/createV2")
            .query("universeId", &experience_id.to_string())
            .query("templatePlaceIdToUse", &95206881.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_string("");

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreatePlaceResponse>()
            .map_err(|e| format!("Failed to deserialize create place response: {}", e))?;

        if !model.success {
            return Err("Failed to create place (unknown error)".to_owned());
        }

        Ok(model)
    }

    pub fn configure_experience(
        &mut self,
        experience_id: AssetId,
        experience_configuration: &ExperienceConfigurationModel,
    ) -> Result<(), String> {
        let json_data = match serde_json::to_value(&experience_configuration) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!(
                    "Failed to serialize experience configuration\n\t{}",
                    e
                ))
            }
        };

        let res = ureq::request(
            "PATCH",
            &format!(
                "https://develop.roblox.com/v2/universes/{}/configuration",
                experience_id
            ),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json_data);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn configure_place(
        &mut self,
        place_id: AssetId,
        place_configuration: &PlaceConfigurationModel,
    ) -> Result<(), String> {
        let json_data = match serde_json::to_value(&place_configuration) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to serialize place configuration\n\t{}", e)),
        };

        let res = ureq::request(
            "PATCH",
            &format!("https://develop.roblox.com/v2/places/{}", place_id),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .set("Content-Type", "application/json")
        .send_json(json_data);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn set_experience_active(
        &mut self,
        experience_id: AssetId,
        active: bool,
    ) -> Result<(), String> {
        let endpoint = if active { "activate" } else { "deactivate" };
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/{}",
            experience_id, endpoint
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }

    fn get_image_from_data(
        file_field_name: String,
        image_file: &Path,
        text_fields: Option<HashMap<String, String>>,
    ) -> Result<PreparedFields, String> {
        let stream = fs::File::open(image_file)
            .map_err(|e| format!("Failed to open image file {}: {}", image_file.display(), e))?;
        let file_name = Some(
            image_file
                .file_name()
                .and_then(OsStr::to_str)
                .ok_or("Unable to determine image name")?,
        );
        let mime = Some(mime_guess::from_path(image_file).first_or_octet_stream());

        let mut multipart = Multipart::new();
        multipart.add_stream(file_field_name, stream, file_name, mime);

        if let Some(fields) = text_fields {
            for (name, text) in fields {
                multipart.add_text(name, text);
            }
        }

        multipart
            .prepare()
            .map_err(|e| format!("Failed to load image file {}: {}", image_file.display(), e))
    }

    pub fn upload_icon(
        &mut self,
        experience_id: AssetId,
        icon_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart = Self::get_image_from_data("request.files".to_owned(), icon_file, None)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/games/{}/icon",
            experience_id
        ))
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send(multipart);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<UploadImageResponse>()
            .map_err(|e| format!("Failed to deserialize upload image response: {}", e))?;

        Ok(model)
    }

    pub fn upload_thumbnail(
        &mut self,
        experience_id: AssetId,
        thumbnail_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart =
            Self::get_image_from_data("request.files".to_owned(), thumbnail_file, None)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/games/{}/thumbnail/image",
            experience_id
        ))
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send(multipart);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<UploadImageResponse>()
            .map_err(|e| format!("Failed to deserialize upload image response: {}", e))?;

        Ok(model)
    }

    pub fn set_experience_thumbnail_order(
        &mut self,
        experience_id: AssetId,
        new_thumbnail_order: &[AssetId],
    ) -> Result<(), String> {
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/order",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({ "thumbnailIds": new_thumbnail_order }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn delete_experience_thumbnail(
        &mut self,
        experience_id: AssetId,
        thumbnail_id: AssetId,
    ) -> Result<(), String> {
        let res = ureq::delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/{}",
            experience_id, thumbnail_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn create_experience_developer_product(
        &mut self,
        experience_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> Result<CreateDeveloperProductResponse, String> {
        let mut req = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/developerproducts",
            experience_id
        ))
        .query("name", &name)
        .query("priceInRobux", &price.to_string())
        .query("description", &description);
        if let Some(icon_asset_id) = icon_asset_id {
            req = req.query("iconAssetId", &icon_asset_id.to_string());
        }
        let res = req
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_string("");

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreateDeveloperProductResponse>()
            .map_err(|e| {
                format!(
                    "Failed to deserialize create experience developer product response: {}",
                    e
                )
            })?;

        Ok(model)
    }

    pub fn list_experience_developer_products(
        &mut self,
        experience_id: AssetId,
        page: u32,
    ) -> Result<ListDeveloperProductsResponse, String> {
        let res = ureq::get(&"https://api.roblox.com/developerproducts/list".to_owned())
            .query("universeId", &experience_id.to_string())
            .query("page", &page.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_string("");

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ListDeveloperProductsResponse>()
            .map_err(|e| {
                format!(
                    "Failed to deserialize create experience developer product response: {}",
                    e
                )
            })?;

        Ok(model)
    }

    pub fn find_experience_developer_product_by_id(
        &mut self,
        experience_id: AssetId,
        developer_product_id: AssetId,
    ) -> Result<GetDeveloperProductResponse, String> {
        let mut page: u32 = 1;
        loop {
            let res = self.list_experience_developer_products(experience_id, page)?;

            let product = res
                .developer_products
                .iter()
                .find(|p| p.developer_product_id == developer_product_id);

            if let Some(product) = product {
                return Ok(product.clone());
            }

            if res.final_page {
                return Err(format!(
                    "Failed to find developer product with id {}",
                    developer_product_id
                ));
            }

            page += 1;
        }
    }

    pub fn update_experience_developer_product(
        &mut self,
        experience_id: AssetId,
        developer_product_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> Result<(), String> {
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/developerproducts/{}/update",
            experience_id, developer_product_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({
            "name": name,
            "priceInRobux": price,
            "description": description,
            "iconAssetId": icon_asset_id
        }));

        Self::handle_response(res)?;

        Ok(())
    }
}
