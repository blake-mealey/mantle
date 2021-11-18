use multipart::client::lazy::{Multipart, PreparedFields};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{clone::Clone, collections::HashMap, ffi::OsStr, fmt, fs, path::Path};
use ureq::{Cookie, Response};
use url::Url;

use crate::{
    resource_manager::AssetId,
    roblox_auth::{AuthType, RequestExt, RobloxAuth},
};

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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CreatorType {
    User,
    Group,
}
impl fmt::Display for CreatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreatorType::User => write!(f, "User"),
            CreatorType::Group => write!(f, "Group"),
        }
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
    pub success: bool,
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
    pub custom_social_slots_count: u32,
    pub is_root_place: bool,
}

impl From<GetPlaceResponse> for PlaceConfigurationModel {
    fn from(response: GetPlaceResponse) -> Self {
        PlaceConfigurationModel {
            name: Some(response.name),
            description: Some(response.description),
            max_player_count: Some(response.max_player_count),
            allow_copying: Some(response.allow_copying),
            social_slot_type: Some(response.social_slot_type),
            custom_social_slot_count: Some(response.custom_social_slots_count),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemovePlaceResponse {
    pub success: bool,
}

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
    pub developer_products: Vec<GetDeveloperProductResponse>,
    pub final_page: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GetDeveloperProductResponse {
    pub product_id: AssetId,
    pub developer_product_id: AssetId,
    pub name: String,
    pub description: Option<String>,
    pub icon_image_asset_id: Option<AssetId>,
    pub price_in_robux: u32,
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
    pub success: bool,
    pub asset_id: AssetId,
    pub backing_asset_id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCreateAudioAssetPriceResponse {
    pub price: u32,
    pub balance: u32,
    pub can_afford: bool,
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
pub struct GetGamesThumbnailsResponse {
    pub data: Vec<GetGameThumbnailResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetGameThumbnailResponse {
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

    pub is_archived: Option<bool>,
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

    fn get_html_input_value(raw_html: &str, selector: &str) -> Result<String, String> {
        let fragment = Html::parse_fragment(raw_html);
        let input_selector = Selector::parse(selector)
            .map_err(|_| format!("Failed to parse selector {}", selector))?;
        let input_element = fragment
            .select(&input_selector)
            .next()
            .ok_or(format!("Failed to find input with selector {}", selector))?;
        let input_value = input_element
            .value()
            .attr("value")
            .ok_or(format!(
                "input with selector {} did not have a value",
                selector
            ))?
            .to_owned();

        Ok(input_value)
    }

    fn get_cookie_value(response: &Response, name: &str) -> Result<String, String> {
        response
            .all("set-cookie")
            .iter()
            .find_map(|c| match Cookie::parse(c.to_owned()) {
                Ok(cookie) if cookie.name() == name => Some(cookie.value().to_owned()),
                _ => None,
            })
            .ok_or(format!("Response did not include a {} cookie", name))
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
        .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetPlaceResponse>()
            .map_err(|e| format!("Failed to deserialize get place response: {}", e))?;

        Ok(model)
    }

    pub fn list_places(
        &mut self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListPlacesResponse, String> {
        let mut req = ureq::get(&format!(
            "https://develop.roblox.com/v1/universes/{}/places",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query("cursor", &page_cursor);
        }
        let res = req
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ListPlacesResponse>()
            .map_err(|e| format!("Failed to deserialize list places response: {}", e))?;

        Ok(model)
    }

    pub fn get_all_places(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<GetPlaceResponse>, String> {
        let mut all_places = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_places(experience_id, page_cursor)?;
            for ListPlaceResponse { id } in res.data {
                let place = self.get_place(id)?;
                all_places.push(place);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_places)
    }

    pub fn remove_place_from_experience(
        &mut self,
        experience_id: AssetId,
        place_id: AssetId,
    ) -> Result<(), String> {
        let res = ureq::post("https://www.roblox.com/universes/removeplace")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_form(&[
                ("universeId", &experience_id.to_string()),
                ("placeId", &place_id.to_string()),
            ]);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<RemovePlaceResponse>()
            .map_err(|e| format!("Failed to deserialize get place response: {}", e))?;

        if !model.success {
            return Err("Failed to remove place from experience (unknown error)".to_owned());
        }

        Ok(())
    }

    pub fn create_experience(
        &mut self,
        group_id: Option<AssetId>,
    ) -> Result<CreateExperienceResponse, String> {
        let res = ureq::post("https://api.roblox.com/universes/create")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "templatePlaceIdToUse": 95206881,
                "groupId": group_id
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
        .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetExperienceResponse>()
            .map_err(|e| format!("Failed to deserialize get experience response: {}", e))?;

        Ok(model)
    }

    pub fn get_experience_configuration(
        &mut self,
        experience_id: AssetId,
    ) -> Result<ExperienceConfigurationModel, String> {
        let res = ureq::get(&format!(
            "https://develop.roblox.com/v1/universes/{}/configuration",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ExperienceConfigurationModel>()
            .map_err(|e| {
                format!(
                    "Failed to deserialize get experience configuration response: {}",
                    e
                )
            })?;

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

    fn internal_create_multipart_form(
        text_fields: Option<HashMap<String, String>>,
    ) -> Multipart<'static, 'static> {
        let mut multipart = Multipart::new();

        if let Some(fields) = text_fields {
            for (name, text) in fields {
                multipart.add_text(name, text);
            }
        }

        multipart
    }

    fn create_multipart_form_from_fields(
        text_fields: HashMap<String, String>,
    ) -> Result<PreparedFields<'static>, String> {
        let mut multipart = Self::internal_create_multipart_form(Some(text_fields));

        multipart
            .prepare()
            .map_err(|e| format!("Failed to create multipart form from fields: {}", e))
    }

    fn create_multipart_form_from_file(
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

        let mut multipart = Self::internal_create_multipart_form(text_fields);

        multipart.add_stream(file_field_name, stream, file_name, mime);

        multipart.prepare().map_err(|e| {
            format!(
                "Failed to create multipart form from image file {}: {}",
                image_file.display(),
                e
            )
        })
    }

    pub fn upload_icon(
        &mut self,
        experience_id: AssetId,
        icon_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart =
            Self::create_multipart_form_from_file("request.files".to_owned(), icon_file, None)?;

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

    pub fn remove_experience_icon(
        &mut self,
        start_place_id: AssetId,
        icon_asset_id: AssetId,
    ) -> Result<(), String> {
        let res = ureq::post("https://www.roblox.com/places/icons/remove-icon")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_form(&[
                ("placeId", &start_place_id.to_string()),
                ("placeIconId", &icon_asset_id.to_string()),
            ]);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn upload_thumbnail(
        &mut self,
        experience_id: AssetId,
        thumbnail_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart = Self::create_multipart_form_from_file(
            "request.files".to_owned(),
            thumbnail_file,
            None,
        )?;

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

    pub fn get_experience_thumbnails(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<GetGameThumbnailResponse>, String> {
        let res = ureq::get(&format!(
            "https://games.roblox.com/v1/games/{}/media",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetGamesThumbnailsResponse>()
            .map_err(|e| format!("Failed to deserialize get game thumbnails response: {}", e))?;

        Ok(model.data)
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

    pub fn create_developer_product_icon(
        &mut self,
        experience_id: AssetId,
        icon_file: &Path,
    ) -> Result<AssetId, String> {
        let (image_verification_token, request_verification_token) = {
            let res = ureq::get("https://www.roblox.com/places/create-developerproduct")
                .query("universeId", &experience_id.to_string())
                .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
                .call();

            let response = Self::handle_response(res)?;
            let request_verification_token =
                Self::get_cookie_value(&response, "__RequestVerificationToken")?;
            let raw_html = response
                .into_string()
                .map_err(|e| format!("Failed to read HTML response: {}", e))?;
            let image_verification_token = Self::get_html_input_value(
                &raw_html,
                "#DeveloperProductImageUpload input[name=\"__RequestVerificationToken\"]",
            )?;

            (image_verification_token, request_verification_token)
        };

        let mut text_fields = HashMap::new();
        text_fields.insert(
            "__RequestVerificationToken".to_owned(),
            image_verification_token,
        );
        let multipart = Self::create_multipart_form_from_file(
            "DeveloperProductImageFile".to_owned(),
            icon_file,
            Some(text_fields),
        )?;

        let res = ureq::post("https://www.roblox.com/places/developerproduct-icon")
            .query("developerProductId", "0")
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", multipart.boundary()),
            )
            .set_auth(
                AuthType::CookieAndCsrfTokenAndVerificationToken {
                    verification_token: request_verification_token,
                },
                &mut self.roblox_auth,
            )?
            .send(multipart);

        let response = Self::handle_response(res)?;
        let raw_html = response
            .into_string()
            .map_err(|e| format!("Failed to read HTML response: {}", e))?;
        let asset_id =
            Self::get_html_input_value(&raw_html, "#developerProductIcon input[id=\"assetId\"]")?
                .parse::<AssetId>()
                .map_err(|e| format!("Failed to parse asset id: {}", e))?;

        Ok(asset_id)
    }

    pub fn create_developer_product(
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
            req = req.query("iconImageAssetId", &icon_asset_id.to_string());
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

    pub fn create_social_link(
        &mut self,
        experience_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> Result<CreateSocialLinkResponse, String> {
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/social-links",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({
            "title": title,
            "url": url,
            "type": link_type,
        }));

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreateSocialLinkResponse>()
            .map_err(|e| format!("Failed to deserialize create social link response: {}", e))?;

        Ok(model)
    }

    pub fn update_social_link(
        &mut self,
        experience_id: AssetId,
        social_link_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> Result<(), String> {
        let res = ureq::request(
            "PATCH",
            &format!(
                "https://develop.roblox.com/v1/universes/{}/social-links/{}",
                experience_id, social_link_id
            ),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({
            "title": title,
            "url": url,
            "type": link_type,
        }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn delete_social_link(
        &mut self,
        experience_id: AssetId,
        social_link_id: AssetId,
    ) -> Result<(), String> {
        let res = ureq::delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/social-links/{}",
            experience_id, social_link_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn list_game_passes(
        &mut self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListGamePassesResponse, String> {
        let mut req = ureq::get(&format!(
            "https://games.roblox.com/v1/games/{}/game-passes",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query("cursor", &page_cursor);
        }
        let res = req
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ListGamePassesResponse>()
            .map_err(|e| format!("Failed to deserialize list game passes response: {}", e))?;

        Ok(model)
    }

    pub fn get_game_pass(&mut self, game_pass_id: AssetId) -> Result<GetGamePassResponse, String> {
        let res = ureq::get("https://api.roblox.com/marketplace/game-pass-product-info")
            .query("gamePassId", &game_pass_id.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

        let response = Self::handle_response(res)?;
        let mut model = response
            .into_json::<GetGamePassResponse>()
            .map_err(|e| format!("Failed to deserialize get game pass response: {}", e))?;
        if model.target_id == 0 {
            model.target_id = game_pass_id;
        }

        Ok(model)
    }

    pub fn get_all_game_passes(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<GetGamePassResponse>, String> {
        let mut all_games = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_game_passes(experience_id, page_cursor)?;
            for ListGamePassResponse { id } in res.data {
                let game_pass = self.get_game_pass(id)?;
                all_games.push(game_pass);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_games)
    }

    pub fn list_developer_products(
        &mut self,
        experience_id: AssetId,
        page: u32,
    ) -> Result<ListDeveloperProductsResponse, String> {
        let res = ureq::get(&"https://api.roblox.com/developerproducts/list".to_owned())
            .query("universeId", &experience_id.to_string())
            .query("page", &page.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

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

    pub fn get_all_developer_products(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<GetDeveloperProductResponse>, String> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_developer_products(experience_id, page)?;
            all_products.extend(res.developer_products);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }

    pub fn find_developer_product_by_id(
        &mut self,
        experience_id: AssetId,
        developer_product_id: AssetId,
    ) -> Result<GetDeveloperProductResponse, String> {
        let mut page: u32 = 1;
        loop {
            let res = self.list_developer_products(experience_id, page)?;

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

    pub fn update_developer_product(
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
            "Name": name,
            "PriceInRobux": price,
            "Description": description,
            "IconImageAssetId": icon_asset_id
        }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn create_game_pass(
        &mut self,
        start_place_id: AssetId,
        name: String,
        description: Option<String>,
        icon_file: &Path,
    ) -> Result<CreateGamePassResponse, String> {
        let (form_verification_token, request_verification_token) = {
            let res = ureq::get("https://www.roblox.com/build/upload")
                .query("assetTypeId", "34")
                .query("targetPlaceId", &start_place_id.to_string())
                .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
                .call();

            let response = Self::handle_response(res)?;
            let request_verification_token =
                Self::get_cookie_value(&response, "__RequestVerificationToken")?;
            let raw_html = response
                .into_string()
                .map_err(|e| format!("Failed to read HTML response: {}", e))?;
            let form_verification_token = Self::get_html_input_value(
                &raw_html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?;

            (form_verification_token, request_verification_token)
        };

        let (form_verification_token, icon_asset_id) = {
            let mut text_fields = HashMap::new();
            text_fields.insert(
                "__RequestVerificationToken".to_owned(),
                form_verification_token,
            );
            text_fields.insert("assetTypeId".to_owned(), "34".to_owned());
            text_fields.insert("targetPlaceId".to_owned(), start_place_id.to_string());
            text_fields.insert("name".to_owned(), name.clone());
            if let Some(description) = description.clone() {
                text_fields.insert("description".to_owned(), description);
            }
            let multipart = Self::create_multipart_form_from_file(
                "file".to_owned(),
                icon_file,
                Some(text_fields),
            )?;

            let res = ureq::post("https://www.roblox.com/build/verifyupload")
                .set(
                    "Content-Type",
                    &format!("multipart/form-data; boundary={}", multipart.boundary()),
                )
                .set_auth(
                    AuthType::CookieAndCsrfTokenAndVerificationToken {
                        verification_token: request_verification_token.clone(),
                    },
                    &mut self.roblox_auth,
                )?
                .send(multipart);

            let response = Self::handle_response(res)?;
            let raw_html = response
                .into_string()
                .map_err(|e| format!("Failed to read HTML response: {}", e))?;
            let form_verification_token = Self::get_html_input_value(
                &raw_html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?;
            let icon_asset_id =
                Self::get_html_input_value(&raw_html, "#upload-form input[name=\"assetImageId\"]")?
                    .parse::<AssetId>()
                    .map_err(|e| format!("Failed to parse asset id: {}", e))?;

            (form_verification_token, icon_asset_id)
        };

        let mut text_fields = HashMap::new();
        text_fields.insert(
            "__RequestVerificationToken".to_owned(),
            form_verification_token,
        );
        text_fields.insert("assetTypeId".to_owned(), "34".to_owned());
        text_fields.insert("targetPlaceId".to_owned(), start_place_id.to_string());
        text_fields.insert("name".to_owned(), name);
        if let Some(description) = description {
            text_fields.insert("description".to_owned(), description);
        }
        text_fields.insert("assetImageId".to_owned(), icon_asset_id.to_string());
        let multipart = Self::create_multipart_form_from_fields(text_fields)?;

        let res = ureq::post("https://www.roblox.com/build/doverifiedupload")
            .set(
                "Content-Type",
                &format!("multipart/form-data; boundary={}", multipart.boundary()),
            )
            .set_auth(
                AuthType::CookieAndCsrfTokenAndVerificationToken {
                    verification_token: request_verification_token,
                },
                &mut self.roblox_auth,
            )?
            .send(multipart);

        let response = Self::handle_response(res)?;

        let location = response.get_url();
        let location_url = Url::parse("https://www.roblox.com")
            .unwrap()
            .join(location)
            .map_err(|e| format!("Failed to parse Location: {}", e))?;
        let asset_id = location_url
            .query_pairs()
            .find_map(|(k, v)| if k == "uploadedId" { Some(v) } else { None })
            .ok_or("Failed to find ID from Location")?
            .parse::<AssetId>()
            .map_err(|e| format!("Failed to parse asset id: {}", e))?;

        Ok(CreateGamePassResponse {
            asset_id,
            icon_asset_id,
        })
    }

    pub fn update_game_pass(
        &mut self,
        game_pass_id: AssetId,
        name: String,
        description: Option<String>,
        price: Option<u32>,
    ) -> Result<(), String> {
        let res = ureq::post("https://www.roblox.com/game-pass/update")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "id":game_pass_id,
                "name": name,
                "description": description,
                "price": price,
                "isForSale": price.is_some(),
            }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn update_game_pass_icon(
        &mut self,
        game_pass_id: AssetId,
        icon_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart =
            Self::create_multipart_form_from_file("request.files".to_owned(), icon_file, None)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/game-passes/{}/icon",
            game_pass_id
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

    pub fn create_badge(
        &mut self,
        experience_id: AssetId,
        name: String,
        description: Option<String>,
        icon_file_path: &Path,
        payment_source: CreatorType,
    ) -> Result<CreateBadgeResponse, String> {
        let mut text_fields = HashMap::new();
        text_fields.insert("request.name".to_owned(), name);
        if let Some(description) = description {
            text_fields.insert("request.description".to_owned(), description);
        }
        text_fields.insert(
            "request.paymentSourceType".to_owned(),
            payment_source.to_string(),
        );
        let multipart = Self::create_multipart_form_from_file(
            "request.files".to_owned(),
            icon_file_path,
            Some(text_fields),
        )?;

        let res = ureq::post(&format!(
            "https://badges.roblox.com/v1/universes/{}/badges",
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
            .into_json::<CreateBadgeResponse>()
            .map_err(|e| format!("Failed to deserialize create badge response: {}", e))?;

        Ok(model)
    }

    pub fn update_badge(
        &mut self,
        badge_id: AssetId,
        name: String,
        description: Option<String>,
        enabled: bool,
    ) -> Result<(), String> {
        let res = ureq::request(
            "PATCH",
            &format!("https://badges.roblox.com/v1/badges/{}", badge_id),
        )
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({
            "name": name,
            "description": description,
            "enabled": enabled,
        }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn list_badges(
        &mut self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListBadgesResponse, String> {
        let mut req = ureq::get(&format!(
            "https://badges.roblox.com/v1/universes/{}/badges",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query("cursor", &page_cursor);
        }
        let res = req
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ListBadgesResponse>()
            .map_err(|e| format!("Failed to deserialize list badges response: {}", e))?;

        Ok(model)
    }

    pub fn get_all_badges(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<ListBadgeResponse>, String> {
        let mut all_badges = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_badges(experience_id, page_cursor)?;
            all_badges.extend(res.data);

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_badges)
    }

    pub fn update_badge_icon(
        &mut self,
        badge_id: AssetId,
        icon_file: &Path,
    ) -> Result<UploadImageResponse, String> {
        let multipart =
            Self::create_multipart_form_from_file("request.files".to_owned(), icon_file, None)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/badges/{}/icon",
            badge_id
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

    pub fn create_asset_alias(
        &mut self,
        experience_id: AssetId,
        asset_id: AssetId,
        name: String,
    ) -> Result<(), String> {
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/aliases",
            experience_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({
            "name": name,
            "type": "1",
            "targetId": asset_id,
        }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn update_asset_alias(
        &mut self,
        experience_id: AssetId,
        asset_id: AssetId,
        previous_name: String,
        name: String,
    ) -> Result<(), String> {
        let res = ureq::post("https://api.roblox.com/universes/update-alias-v2")
            .query("universeId", &experience_id.to_string())
            .query("oldName", &previous_name)
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "name": name,
                "type": "1",
                "targetId": asset_id,
            }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn delete_asset_alias(
        &mut self,
        experience_id: AssetId,
        name: String,
    ) -> Result<(), String> {
        let res = ureq::post("https://api.roblox.com/universes/delete-alias")
            .query("universeId", &experience_id.to_string())
            .query("name", &name)
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn list_asset_aliases(
        &mut self,
        experience_id: AssetId,
        page: u32,
    ) -> Result<ListAssetAliasesResponse, String> {
        let res = ureq::get(&"https://api.roblox.com/universes/get-aliases".to_owned())
            .query("universeId", &experience_id.to_string())
            .query("page", &page.to_string())
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .call();

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<ListAssetAliasesResponse>()
            .map_err(|e| format!("Failed to deserialize list asset aliases response: {}", e))?;

        Ok(model)
    }

    pub fn get_all_asset_aliases(
        &mut self,
        experience_id: AssetId,
    ) -> Result<Vec<GetAssetAliasResponse>, String> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_asset_aliases(experience_id, page)?;
            all_products.extend(res.aliases);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }

    pub fn create_image_asset(
        &mut self,
        file_path: &Path,
        group_id: Option<AssetId>,
    ) -> Result<CreateImageAssetResponse, String> {
        let data = fs::read(file_path).map_err(|e| {
            format!(
                "Unable to read image asset file: {}\n\t{}",
                file_path.display(),
                e
            )
        })?;

        let file_name = format!(
            "Images/{}",
            file_path.file_stem().map(OsStr::to_str).flatten().unwrap()
        );
        let mut req = ureq::post("https://data.roblox.com/data/upload/json")
            .query("assetTypeId", "13")
            .query("name", &file_name)
            .query("description", "madewithmantle");
        if let Some(group_id) = group_id {
            req = req.query("groupId", &group_id.to_string());
        }
        let res = req
            .set("Content-Type", "*/*")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_bytes(&data);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreateImageAssetResponse>()
            .map_err(|e| format!("Failed to deserialize create image asset response: {}", e))?;

        if !model.success {
            return Err("Failed to create image asset (unknown error)".to_owned());
        }

        Ok(model)
    }

    pub fn get_create_audio_asset_price(
        &mut self,
        file_path: &Path,
        group_id: Option<AssetId>,
    ) -> Result<GetCreateAudioAssetPriceResponse, String> {
        let data = fs::read(file_path).map_err(|e| {
            format!(
                "Unable to read audio asset file: {}\n\t{}",
                file_path.display(),
                e
            )
        })?;

        let file_name = format!(
            "Audio/{}",
            file_path.file_stem().map(OsStr::to_str).flatten().unwrap()
        );

        let res = ureq::post("https://publish.roblox.com/v1/audio/verify")
            .query("name", &file_name)
            .set("Content-Type", "*/*")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "name": file_name,
                "fileSize": data.len(),
                "file": base64::encode(data),
                "groupId": group_id,
            }));

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<GetCreateAudioAssetPriceResponse>()
            .map_err(|e| {
                format!(
                    "Failed to deserialize get create audio asset price response: {}",
                    e
                )
            })?;

        Ok(model)
    }

    pub fn create_audio_asset(
        &mut self,
        file_path: &Path,
        group_id: Option<AssetId>,
        payment_source: CreatorType,
    ) -> Result<CreateAudioAssetResponse, String> {
        let data = fs::read(file_path).map_err(|e| {
            format!(
                "Unable to read audio asset file: {}\n\t{}",
                file_path.display(),
                e
            )
        })?;

        let file_name = format!(
            "Audio/{}",
            file_path.file_stem().map(OsStr::to_str).flatten().unwrap()
        );
        let res = ureq::post("https://publish.roblox.com/v1/audio")
            .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
            .send_json(json!({
                "name": file_name,
                "file": base64::encode(data),
                "groupId": group_id,
                "paymentSource": payment_source
            }));

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<CreateAudioAssetResponse>()
            .map_err(|e| format!("Failed to deserialize create audio asset response: {}", e))?;

        Ok(model)
    }

    pub fn archive_asset(&mut self, asset_id: AssetId) -> Result<(), String> {
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/assets/{}/archive",
            asset_id
        ))
        .set_auth(AuthType::CookieAndCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }
}
