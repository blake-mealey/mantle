use std::{clone::Clone, ffi::OsStr, fs, path::PathBuf, str, sync::Arc};

use reqwest::{
    header,
    multipart::{Form as MultipartForm, Part},
    Body, StatusCode,
};
use scraper::{Html, Selector};
use serde::{de, Deserialize, Serialize};
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use url::Url;

use crate::{roblox_auth::RobloxAuth, roblox_resource_manager::AssetId};

#[derive(Deserialize, Debug)]
struct RobloxApiErrorModel {
    // There are some other possible properties but we currently have no use for them so they are not
    // included

    // Most error models have a `message` property
    #[serde(alias = "Message")]
    message: Option<String>,

    // Some error models (500) have a `title` property instead
    #[serde(alias = "Title")]
    title: Option<String>,

    // Some error models on older APIs have an errors array
    #[serde(alias = "Errors")]
    errors: Option<Vec<RobloxApiErrorModel>>,

    // Some errors return a `success` property which can be used to check for errors
    #[serde(alias = "Success")]
    success: Option<bool>,
}

impl RobloxApiErrorModel {
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
struct RemovePlaceResponse {}

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
            name: "Untitled Game".to_owned(),
            description: "Created with Mantle".to_owned(),
            max_player_count: 50,
            allow_copying: false,
            social_slot_type: SocialSlotType::Automatic,
            custom_social_slots_count: None,
        }
    }
}

enum PlaceFileFormat {
    Xml,
    Binary,
}

pub struct RobloxApi {
    client: reqwest::Client,
}

impl RobloxApi {
    pub async fn new(roblox_auth: RobloxAuth) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .user_agent("Roblox/WinInet")
            .cookie_provider(Arc::new(roblox_auth.jar))
            .default_headers(roblox_auth.headers)
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
        let roblox_api = Self { client };
        roblox_api.validate_auth().await?;
        Ok(roblox_api)
    }

    async fn get_roblox_api_error_message_from_response(response: reqwest::Response) -> String {
        let status_code = response.status();
        let reason = {
            if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
                if content_type == "application/json" {
                    match response.json::<RobloxApiErrorModel>().await {
                        Ok(error) => Some(error.reason_or_status_code(status_code)),
                        Err(_) => None,
                    }
                } else if content_type == "text/html" {
                    // println!("{}", response.text().await.unwrap());
                    None
                } else {
                    response.text().await.ok()
                }
            } else {
                None
            }
        };
        reason.unwrap_or_else(|| format!("Unknown error (status {})", status_code))
    }

    async fn handle(request_builder: reqwest::RequestBuilder) -> Result<reqwest::Response, String> {
        let result = request_builder.send().await;
        match result {
            Ok(response) => {
                // Check for redirects to the login page
                let url = response.url();
                if matches!(url.domain(), Some("www.roblox.com")) && url.path() == "/NewLogin" {
                    return Err("Authorization has been denied for this request.".to_owned());
                }

                // Check status code
                if response.status().is_success() {
                    Ok(response)
                } else {
                    Err(Self::get_roblox_api_error_message_from_response(response).await)
                }
            }
            Err(error) => Err(format!("HTTP client error: {}", error)),
        }
    }

    async fn handle_as_json<T>(request_builder: reqwest::RequestBuilder) -> Result<T, String>
    where
        T: de::DeserializeOwned,
    {
        Self::handle(request_builder)
            .await?
            .json::<T>()
            .await
            .map_err(|e| format!("Failed to deserialize response: {}", e))
    }

    async fn handle_as_json_with_status<T>(
        request_builder: reqwest::RequestBuilder,
    ) -> Result<T, String>
    where
        T: de::DeserializeOwned,
    {
        let response = Self::handle(request_builder).await?;
        let status_code = response.status();
        let data = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        if let Ok(error) = serde_json::from_slice::<RobloxApiErrorModel>(&data) {
            if !error.success.unwrap_or(false) {
                return Err(error.reason_or_status_code(status_code));
            }
        }
        serde_json::from_slice::<T>(&data)
            .map_err(|e| format!("Failed to deserialize response: {}", e))
    }

    async fn handle_as_html(request_builder: reqwest::RequestBuilder) -> Result<Html, String> {
        let text = Self::handle(request_builder)
            .await?
            .text()
            .await
            .map_err(|e| format!("Failed to read HTML response: {}", e))?;
        Ok(Html::parse_fragment(&text))
    }

    async fn get_file_part(file_path: PathBuf) -> Result<Part, String> {
        let file = File::open(&file_path)
            .await
            .map_err(|e| format!("Failed to open image file {}: {}", file_path.display(), e))?;
        let reader = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));

        let file_name = file_path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or("Unable to determine image file name")?
            .to_owned();
        let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

        Ok(Part::stream(reader)
            .file_name(file_name)
            .mime_str(&mime.to_string())
            .unwrap())
    }

    fn get_input_value(html: &Html, selector: &str) -> Result<String, String> {
        let input_selector = Selector::parse(selector)
            .map_err(|_| format!("Failed to parse selector {}", selector))?;
        let input_element = html
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

    pub async fn validate_auth(&self) -> Result<(), String> {
        let req = self
            .client
            .get("https://users.roblox.com/v1/users/authenticated");

        Self::handle(req).await.map_err(|_| {
            "Authorization validation failed. Check your ROBLOSECURITY cookie.".to_owned()
        })?;

        Ok(())
    }

    pub async fn upload_place(&self, place_file: PathBuf, place_id: AssetId) -> Result<(), String> {
        let extension = place_file.extension().ok_or(format!(
            "No file extension on place file {} (expected .rbxl or .rbxlx).",
            place_file.display()
        ))?;

        let file_format = match extension.to_str() {
            Some("rbxl") => PlaceFileFormat::Binary,
            Some("rbxlx") => PlaceFileFormat::Xml,
            _ => {
                return Err(format!(
                    "Unknown file extension on place file {} (expected .rbxl or .rbxlx).",
                    place_file.display()
                ))
            }
        };

        let data = fs::read(&place_file).map_err(|e| {
            format!(
                "Unable to read place file: {}\n\t{}",
                place_file.display(),
                e
            )
        })?;

        let body: Body = match file_format {
            PlaceFileFormat::Binary => data.into(),
            PlaceFileFormat::Xml => String::from_utf8(data)
                .map_err(|_| "Unable to read place file")?
                .into(),
        };

        let content_type = match file_format {
            PlaceFileFormat::Binary => "application/octet-stream",
            PlaceFileFormat::Xml => "application/xml",
        };

        let req = self
            .client
            .post("https://data.roblox.com/Data/Upload.ashx")
            .query(&[("assetId", place_id.to_string())])
            .header("Content-Type", content_type)
            .body(body);

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn get_place(&self, place_id: AssetId) -> Result<GetPlaceResponse, String> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v2/places/{}",
            place_id
        ));

        Self::handle_as_json(req).await
    }

    pub async fn list_places(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListPlacesResponse, String> {
        let mut req = self.client.get(format!(
            "https://develop.roblox.com/v1/universes/{}/places",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        Self::handle_as_json(req).await
    }

    // TODO: implement generic form
    pub async fn get_all_places(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetPlaceResponse>, String> {
        let mut all_places = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_places(experience_id, page_cursor).await?;
            for ListPlaceResponse { id } in res.data {
                let place = self.get_place(id).await?;
                all_places.push(place);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_places)
    }

    pub async fn remove_place_from_experience(
        &self,
        experience_id: AssetId,
        place_id: AssetId,
    ) -> Result<(), String> {
        let req = self
            .client
            .post("https://www.roblox.com/universes/removeplace")
            .form(&[
                ("universeId", &experience_id.to_string()),
                ("placeId", &place_id.to_string()),
            ]);

        Self::handle_as_json_with_status::<RemovePlaceResponse>(req).await?;

        Ok(())
    }

    pub async fn create_experience(
        &self,
        group_id: Option<AssetId>,
    ) -> Result<CreateExperienceResponse, String> {
        let req = self
            .client
            .post("https://api.roblox.com/universes/create")
            .json(&json!({
                "templatePlaceIdToUse": 95206881,
                "groupId": group_id
            }));

        Self::handle_as_json(req).await
    }

    pub async fn get_experience(
        &self,
        experience_id: AssetId,
    ) -> Result<GetExperienceResponse, String> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}",
            experience_id
        ));

        Self::handle_as_json(req).await
    }

    pub async fn get_experience_configuration(
        &self,
        experience_id: AssetId,
    ) -> Result<ExperienceConfigurationModel, String> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}/configuration",
            experience_id
        ));

        Self::handle_as_json(req).await
    }

    pub async fn create_place(
        &self,
        experience_id: AssetId,
    ) -> Result<CreatePlaceResponse, String> {
        let req = self
            .client
            .post("https://www.roblox.com/ide/places/createV2")
            .header(header::CONTENT_LENGTH, 0)
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("templatePlaceIdToUse", &95206881.to_string()),
            ]);

        Self::handle_as_json_with_status(req).await
    }

    pub async fn configure_experience(
        &self,
        experience_id: AssetId,
        experience_configuration: &ExperienceConfigurationModel,
    ) -> Result<(), String> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v2/universes/{}/configuration",
                experience_id
            ))
            .json(experience_configuration);

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn configure_place(
        &self,
        place_id: AssetId,
        place_configuration: &PlaceConfigurationModel,
    ) -> Result<(), String> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v2/places/{}",
                place_id
            ))
            .json(place_configuration);

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn set_experience_active(
        &self,
        experience_id: AssetId,
        active: bool,
    ) -> Result<(), String> {
        let endpoint = if active { "activate" } else { "deactivate" };
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/{}",
                experience_id, endpoint
            ))
            .header(header::CONTENT_LENGTH, 0);

        Self::handle(req).await?;

        Ok(())
    }

    // TODO: Generic form
    pub async fn upload_icon(
        &self,
        experience_id: AssetId,
        icon_file: PathBuf,
    ) -> Result<UploadImageResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/games/{}/icon",
                experience_id
            ))
            .multipart(
                MultipartForm::new().part("request.files", Self::get_file_part(icon_file).await?),
            );

        Self::handle_as_json(req).await
    }

    pub async fn remove_experience_icon(
        &self,
        start_place_id: AssetId,
        icon_asset_id: AssetId,
    ) -> Result<(), String> {
        let req = self
            .client
            .post("https://www.roblox.com/places/icons/remove-icon")
            .form(&[
                ("placeId", &start_place_id.to_string()),
                ("placeIconId", &icon_asset_id.to_string()),
            ]);

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn upload_thumbnail(
        &self,
        experience_id: AssetId,
        thumbnail_file: PathBuf,
    ) -> Result<UploadImageResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/games/{}/thumbnail/image",
                experience_id
            ))
            .multipart(
                MultipartForm::new()
                    .part("request.files", Self::get_file_part(thumbnail_file).await?),
            );

        Self::handle_as_json(req).await
    }

    pub async fn get_experience_thumbnails(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetExperienceThumbnailResponse>, String> {
        let req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/media",
            experience_id
        ));

        Ok(Self::handle_as_json::<GetExperienceThumbnailsResponse>(req)
            .await?
            .data)
    }

    pub async fn set_experience_thumbnail_order(
        &self,
        experience_id: AssetId,
        new_thumbnail_order: &[AssetId],
    ) -> Result<(), String> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/thumbnails/order",
                experience_id
            ))
            .json(&json!({ "thumbnailIds": new_thumbnail_order }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn delete_experience_thumbnail(
        &self,
        experience_id: AssetId,
        thumbnail_id: AssetId,
    ) -> Result<(), String> {
        let req = self.client.delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/{}",
            experience_id, thumbnail_id
        ));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn create_developer_product_icon(
        &self,
        experience_id: AssetId,
        icon_file: PathBuf,
    ) -> Result<AssetId, String> {
        let image_verification_token = {
            let req = self
                .client
                .get("https://www.roblox.com/places/create-developerproduct")
                .query(&[("universeId", &experience_id.to_string())]);

            let html = Self::handle_as_html(req).await?;
            Self::get_input_value(
                &html,
                "#DeveloperProductImageUpload input[name=\"__RequestVerificationToken\"]",
            )?
        };

        let req = self
            .client
            .post("https://www.roblox.com/places/developerproduct-icon")
            .query(&[("developerProductId", "0")])
            .multipart(
                MultipartForm::new()
                    .part(
                        "DeveloperProductImageFile",
                        Self::get_file_part(icon_file).await?,
                    )
                    .text("__RequestVerificationToken", image_verification_token),
            );

        let html = Self::handle_as_html(req).await?;

        Self::get_input_value(&html, "#developerProductIcon input[id=\"assetId\"]")?
            .parse()
            .map_err(|e| format!("Failed to parse asset id: {}", e))
    }

    pub async fn create_developer_product(
        &self,
        experience_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> Result<CreateDeveloperProductResponse, String> {
        let mut req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/developerproducts",
                experience_id
            ))
            .header(header::CONTENT_LENGTH, 0)
            .query(&[
                ("name", &name),
                ("priceInRobux", &price.to_string()),
                ("description", &description),
            ]);
        if let Some(icon_asset_id) = icon_asset_id {
            req = req.query(&[("iconImageAssetId", &icon_asset_id.to_string())]);
        }

        Self::handle_as_json(req).await
    }

    pub async fn create_social_link(
        &self,
        experience_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> Result<CreateSocialLinkResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links",
                experience_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        Self::handle_as_json(req).await
    }

    pub async fn update_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> Result<(), String> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links/{}",
                experience_id, social_link_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn delete_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
    ) -> Result<(), String> {
        let req = self.client.delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/social-links/{}",
            experience_id, social_link_id
        ));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn list_social_links(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetSocialLinkResponse>, String> {
        let req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/social-links/list",
            experience_id
        ));

        Ok(Self::handle_as_json::<ListSocialLinksResponse>(req)
            .await?
            .data)
    }

    pub async fn list_game_passes(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListGamePassesResponse, String> {
        let mut req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/game-passes",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        Self::handle_as_json(req).await
    }

    pub async fn get_game_pass(
        &self,
        game_pass_id: AssetId,
    ) -> Result<GetGamePassResponse, String> {
        let req = self
            .client
            .get("https://api.roblox.com/marketplace/game-pass-product-info")
            .query(&[("gamePassId", &game_pass_id.to_string())]);

        let mut model = Self::handle_as_json::<GetGamePassResponse>(req).await?;
        if model.target_id == 0 {
            model.target_id = game_pass_id;
        }

        Ok(model)
    }

    pub async fn get_all_game_passes(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetGamePassResponse>, String> {
        let mut all_games = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_game_passes(experience_id, page_cursor).await?;
            for ListGamePassResponse { id } in res.data {
                let game_pass = self.get_game_pass(id).await?;
                all_games.push(game_pass);
            }

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_games)
    }

    pub async fn list_developer_products(
        &self,
        experience_id: AssetId,
        page: u32,
    ) -> Result<ListDeveloperProductsResponse, String> {
        let req = self
            .client
            .get("https://api.roblox.com/developerproducts/list")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("page", &page.to_string()),
            ]);

        Self::handle_as_json(req).await
    }

    pub async fn get_all_developer_products(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetDeveloperProductResponse>, String> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_developer_products(experience_id, page).await?;
            all_products.extend(res.developer_products);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }

    pub async fn find_developer_product_by_id(
        &self,
        experience_id: AssetId,
        developer_product_id: AssetId,
    ) -> Result<GetDeveloperProductResponse, String> {
        let mut page: u32 = 1;
        loop {
            let res = self.list_developer_products(experience_id, page).await?;

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

    pub async fn update_developer_product(
        &self,
        experience_id: AssetId,
        developer_product_id: AssetId,
        name: String,
        price: u32,
        description: String,
        icon_asset_id: Option<AssetId>,
    ) -> Result<(), String> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/developerproducts/{}/update",
                experience_id, developer_product_id
            ))
            .json(&json!({
                "Name": name,
                "PriceInRobux": price,
                "Description": description,
                "IconImageAssetId": icon_asset_id
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn create_game_pass(
        &self,
        start_place_id: AssetId,
        name: String,
        description: String,
        icon_file: PathBuf,
    ) -> Result<CreateGamePassResponse, String> {
        let form_verification_token = {
            let req = self
                .client
                .get("https://www.roblox.com/build/upload")
                .query(&[
                    ("assetTypeId", "34"),
                    ("targetPlaceId", &start_place_id.to_string()),
                ]);

            let html = Self::handle_as_html(req).await?;
            Self::get_input_value(
                &html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?
        };

        let (form_verification_token, icon_asset_id) = {
            let req = self
                .client
                .post("https://www.roblox.com/build/verifyupload")
                .multipart(
                    MultipartForm::new()
                        .part("file", Self::get_file_part(icon_file).await?)
                        .text("__RequestVerificationToken", form_verification_token)
                        .text(
                            "assetTypeId",
                            serde_json::to_string(&AssetTypeId::GamePass).unwrap(),
                        )
                        .text("targetPlaceId", start_place_id.to_string())
                        .text("name", name.clone())
                        .text("description", description.clone()),
                );

            let html = Self::handle_as_html(req).await?;
            let form_verification_token = Self::get_input_value(
                &html,
                "#upload-form input[name=\"__RequestVerificationToken\"]",
            )?;
            let icon_asset_id =
                Self::get_input_value(&html, "#upload-form input[name=\"assetImageId\"]")?
                    .parse::<AssetId>()
                    .map_err(|e| format!("Failed to parse asset id: {}", e))?;

            (form_verification_token, icon_asset_id)
        };

        let req = self
            .client
            .post("https://www.roblox.com/build/doverifiedupload")
            .multipart(
                MultipartForm::new()
                    .text("__RequestVerificationToken", form_verification_token)
                    .text(
                        "assetTypeId",
                        serde_json::to_string(&AssetTypeId::GamePass).unwrap(),
                    )
                    .text("targetPlaceId", start_place_id.to_string())
                    .text("name", name)
                    .text("description", description)
                    .text("assetImageId", icon_asset_id.to_string()),
            );

        let response = Self::handle(req).await?;

        let location = response.url();
        let asset_id = location
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

    pub async fn update_game_pass(
        &self,
        game_pass_id: AssetId,
        name: String,
        description: String,
        price: Option<u32>,
    ) -> Result<(), String> {
        let req = self
            .client
            .post("https://www.roblox.com/game-pass/update")
            .json(&json!({
                "id":game_pass_id,
                "name": name,
                "description": description,
                "price": price,
                "isForSale": price.is_some(),
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn update_game_pass_icon(
        &self,
        game_pass_id: AssetId,
        icon_file: PathBuf,
    ) -> Result<UploadImageResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/game-passes/{}/icon",
                game_pass_id
            ))
            .multipart(
                MultipartForm::new().part("request.files", Self::get_file_part(icon_file).await?),
            );

        Self::handle_as_json(req).await
    }

    pub async fn create_badge(
        &self,
        experience_id: AssetId,
        name: String,
        description: String,
        icon_file_path: PathBuf,
        payment_source: CreatorType,
    ) -> Result<CreateBadgeResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://badges.roblox.com/v1/universes/{}/badges",
                experience_id
            ))
            .multipart(
                MultipartForm::new()
                    .part("request.files", Self::get_file_part(icon_file_path).await?)
                    .text("request.name", name)
                    .text("request.description", description)
                    .text(
                        "request.paymentSourceType",
                        serde_json::to_string(&payment_source).unwrap(),
                    ),
            );

        Self::handle_as_json(req).await
    }

    pub async fn update_badge(
        &self,
        badge_id: AssetId,
        name: String,
        description: String,
        enabled: bool,
    ) -> Result<(), String> {
        let req = self
            .client
            .patch(&format!("https://badges.roblox.com/v1/badges/{}", badge_id))
            .json(&json!({
                "name": name,
                "description": description,
                "enabled": enabled,
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn list_badges(
        &self,
        experience_id: AssetId,
        page_cursor: Option<String>,
    ) -> Result<ListBadgesResponse, String> {
        let mut req = self.client.get(&format!(
            "https://badges.roblox.com/v1/universes/{}/badges",
            experience_id
        ));
        if let Some(page_cursor) = page_cursor {
            req = req.query(&[("cursor", &page_cursor)]);
        }

        Self::handle_as_json(req).await
    }

    pub async fn get_all_badges(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<ListBadgeResponse>, String> {
        let mut all_badges = Vec::new();

        let mut page_cursor: Option<String> = None;
        loop {
            let res = self.list_badges(experience_id, page_cursor).await?;
            all_badges.extend(res.data);

            if res.next_page_cursor.is_none() {
                break;
            }

            page_cursor = res.next_page_cursor;
        }

        Ok(all_badges)
    }

    pub async fn update_badge_icon(
        &self,
        badge_id: AssetId,
        icon_file: PathBuf,
    ) -> Result<UploadImageResponse, String> {
        let req = self
            .client
            .post(&format!(
                "https://publish.roblox.com/v1/badges/{}/icon",
                badge_id
            ))
            .multipart(
                MultipartForm::new().part("request.files", Self::get_file_part(icon_file).await?),
            );

        Self::handle_as_json(req).await
    }

    pub async fn create_asset_alias(
        &self,
        experience_id: AssetId,
        asset_id: AssetId,
        name: String,
    ) -> Result<(), String> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/aliases",
                experience_id
            ))
            .json(&json!({
                "name": name,
                "type": "1",
                "targetId": asset_id,
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn update_asset_alias(
        &self,
        experience_id: AssetId,
        asset_id: AssetId,
        previous_name: String,
        name: String,
    ) -> Result<(), String> {
        let req = self
            .client
            .post("https://api.roblox.com/universes/update-alias-v2")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("oldName", &previous_name),
            ])
            .json(&json!({
                "name": name,
                "type": "1",
                "targetId": asset_id,
            }));

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn delete_asset_alias(
        &self,
        experience_id: AssetId,
        name: String,
    ) -> Result<(), String> {
        let req = self
            .client
            .post("https://api.roblox.com/universes/delete-alias")
            .header(header::CONTENT_LENGTH, 0)
            .query(&[("universeId", &experience_id.to_string()), ("name", &name)]);

        Self::handle(req).await?;

        Ok(())
    }

    pub async fn list_asset_aliases(
        &self,
        experience_id: AssetId,
        page: u32,
    ) -> Result<ListAssetAliasesResponse, String> {
        let req = self
            .client
            .get("https://api.roblox.com/universes/get-aliases")
            .query(&[
                ("universeId", &experience_id.to_string()),
                ("page", &page.to_string()),
            ]);

        Self::handle_as_json(req).await
    }

    pub async fn get_all_asset_aliases(
        &self,
        experience_id: AssetId,
    ) -> Result<Vec<GetAssetAliasResponse>, String> {
        let mut all_products = Vec::new();

        let mut page: u32 = 1;
        loop {
            let res = self.list_asset_aliases(experience_id, page).await?;
            all_products.extend(res.aliases);

            if res.final_page {
                break;
            }

            page += 1;
        }

        Ok(all_products)
    }

    pub async fn create_image_asset(
        &self,
        file_path: PathBuf,
        group_id: Option<AssetId>,
    ) -> Result<CreateImageAssetResponse, String> {
        let data = fs::read(&file_path).map_err(|e| {
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

        let mut req = self
            .client
            .post("https://data.roblox.com/data/upload/json")
            .header(reqwest::header::CONTENT_TYPE, "*/*")
            .body(data)
            .query(&[
                ("assetTypeId", "13"),
                ("name", &file_name),
                ("description", "madewithmantle"),
            ]);
        if let Some(group_id) = group_id {
            req = req.query(&[("groupId", &group_id.to_string())]);
        }

        Self::handle_as_json_with_status(req).await
    }

    pub async fn get_create_audio_asset_price(
        &self,
        file_path: PathBuf,
        group_id: Option<AssetId>,
    ) -> Result<GetCreateAudioAssetPriceResponse, String> {
        let data = fs::read(&file_path).map_err(|e| {
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

        let req = self
            .client
            .post("https://publish.roblox.com/v1/audio/verify")
            .query(&[("name", &file_name)])
            .header(reqwest::header::CONTENT_TYPE, "*/*")
            .json(&json!({
                "name": file_name,
                "fileSize": data.len(),
                "file": base64::encode(data),
                "groupId": group_id,
            }));

        Self::handle_as_json(req).await
    }

    pub async fn create_audio_asset(
        &self,
        file_path: PathBuf,
        group_id: Option<AssetId>,
        payment_source: CreatorType,
    ) -> Result<CreateAudioAssetResponse, String> {
        let data = fs::read(&file_path).map_err(|e| {
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

        let req = self
            .client
            .post("https://publish.roblox.com/v1/audio")
            .json(&json!({
                "name": file_name,
                "file": base64::encode(data),
                "groupId": group_id,
                "paymentSource": payment_source
            }));

        Self::handle_as_json(req).await
    }

    pub async fn archive_asset(&self, asset_id: AssetId) -> Result<(), String> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/assets/{}/archive",
                asset_id
            ))
            .header(header::CONTENT_LENGTH, 0);

        Self::handle(req).await?;

        Ok(())
    }
}
