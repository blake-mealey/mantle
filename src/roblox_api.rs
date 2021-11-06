use multipart::client::lazy::{Multipart, PreparedFields};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{clone::Clone, default, ffi::OsStr, fmt, fs, path::Path};

use crate::roblox_auth::RobloxAuth;

enum AuthType {
    ApiKey,
    CookieWithCsrfToken,
}

trait RequestExt {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String>;
}

impl RequestExt for ureq::Request {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String> {
        match auth_type {
            AuthType::ApiKey => Ok(self.set("x-api-key", &auth.get_api_key()?)),
            AuthType::CookieWithCsrfToken => Ok(self
                .set(
                    "cookie",
                    &format!(".ROBLOSECURITY={}", auth.get_roblosecurity()?),
                )
                .set("x-csrf-token", &auth.get_csrf_token()?)),
        }
    }
}

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

enum ProjectType {
    Xml,
    Binary,
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
#[serde(rename_all = "camelCase")]
struct PlaceManagementResponse {
    version_number: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadImageResponse {
    target_id: u64,
}

pub static INVALID_API_KEY_HELP: &str = "\
    Please check your ROBLOX_API_KEY environment variable. \n\
    \tIf you don't have an API key, you can create one at https://create.roblox.com/credentials \n\
    \tYou must ensure that your API key has enabled the 'Place Management API System' and you have \n\
    \tadded the place you are trying to upload to the API System Configuration. You also must ensure \n\
    \tthat your API key's IP whitelist includes the machine you are running this on. You can set it \n\
    \tto '0.0.0.0/0' to whitelist all IPs but this should only be used for testing purposes.";

pub struct UploadPlaceResult {
    pub place_version: u32,
}

pub struct UploadImageResult {
    pub asset_id: u64,
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
        let is_json = response.content_type() == "application/json";

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

        if is_json {
            match response.into_json::<RobloxApiErrorModel>() {
                Ok(v) => get_message_from_error(v),
                Err(_) => None,
            }
        } else {
            response.into_string().ok()
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

    pub fn upload_place(
        &mut self,
        place_file: &Path,
        experience_id: u64,
        place_id: u64,
        mode: DeployMode,
    ) -> Result<UploadPlaceResult, String> {
        // println!("TRACE: upload_place {}", place_file.display());

        let project_type = match place_file.extension().and_then(OsStr::to_str) {
            Some("rbxlx") => ProjectType::Xml,
            Some("rbxl") => ProjectType::Binary,
            Some(v) => return Err(format!("Invalid project file extension: {}", v)),
            None => {
                return Err(format!(
                    "No project file extension in project file: {}",
                    place_file.display()
                ))
            }
        };

        let content_type = match project_type {
            ProjectType::Xml => "application/xml",
            ProjectType::Binary => "application/octet-stream",
        };

        let version_type = match mode {
            DeployMode::Publish => "Published",
            DeployMode::Save => "Saved",
        };

        let req = ureq::post(&format!(
            "https://apis.roblox.com/universes/v1/{}/places/{}/versions",
            experience_id, place_id
        ))
        .set_auth(AuthType::ApiKey, &mut self.roblox_auth)?
        .set("Content-Type", content_type)
        .query("versionType", version_type);

        let res = match project_type {
            ProjectType::Xml => {
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
                req.send_string(&data)
            }
            ProjectType::Binary => {
                let data = match fs::read(place_file) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Unable to read place file: {}\n\t{}",
                            place_file.display(),
                            e
                        ))
                    }
                };
                req.send_bytes(&data)
            }
        };

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<PlaceManagementResponse>()
            .map_err(|e| format!("Failed to deserialize upload place response: {}", e))?;
        Ok(UploadPlaceResult {
            place_version: model.version_number,
        })
    }

    pub fn configure_experience(
        &mut self,
        experience_id: u64,
        experience_configuration: &ExperienceConfigurationModel,
    ) -> Result<(), String> {
        // println!("TRACE: configure_experience {}", experience_id);

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
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send_json(json_data);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn configure_place(
        &mut self,
        place_id: u64,
        place_configuration: &PlaceConfigurationModel,
    ) -> Result<(), String> {
        // println!("TRACE: configure_place {}", place_id);

        let json_data = match serde_json::to_value(&place_configuration) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to serialize place configuration\n\t{}", e)),
        };

        let res = ureq::request(
            "PATCH",
            &format!("https://develop.roblox.com/v2/places/{}", place_id),
        )
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .set("Content-Type", "application/json")
        .send_json(json_data);

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn set_experience_active(
        &mut self,
        experience_id: u64,
        active: bool,
    ) -> Result<(), String> {
        // println!("TRACE: set_experience_active {}", active);

        let endpoint = if active { "activate" } else { "deactivate" };
        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/{}",
            experience_id, endpoint
        ))
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }

    fn get_image_from_data(image_file: &Path) -> Result<PreparedFields, String> {
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
        multipart.add_stream("request.files", stream, file_name, mime);

        multipart
            .prepare()
            .map_err(|e| format!("Failed to load image file {}: {}", image_file.display(), e))
    }

    pub fn upload_icon(
        &mut self,
        experience_id: u64,
        icon_file: &Path,
    ) -> Result<UploadImageResult, String> {
        // println!("TRACE: upload_icon {}", icon_file.display());

        let multipart = Self::get_image_from_data(icon_file)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/games/{}/icon",
            experience_id
        ))
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send(multipart);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<UploadImageResponse>()
            .map_err(|e| format!("Failed to deserialize upload image response: {}", e))?;

        Ok(UploadImageResult {
            asset_id: model.target_id,
        })
    }

    pub fn upload_thumbnail(
        &mut self,
        experience_id: u64,
        thumbnail_file: &Path,
    ) -> Result<UploadImageResult, String> {
        // println!("TRACE: upload_thumbnail {}", thumbnail_file.display());

        let multipart = Self::get_image_from_data(thumbnail_file)?;

        let res = ureq::post(&format!(
            "https://publish.roblox.com/v1/games/{}/thumbnail/image",
            experience_id
        ))
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", multipart.boundary()),
        )
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send(multipart);

        let response = Self::handle_response(res)?;
        let model = response
            .into_json::<UploadImageResponse>()
            .map_err(|e| format!("Failed to deserialize upload image response: {}", e))?;

        Ok(UploadImageResult {
            asset_id: model.target_id,
        })
    }

    pub fn set_experience_thumbnail_order(
        &mut self,
        experience_id: u64,
        new_thumbnail_order: &[u64],
    ) -> Result<(), String> {
        // println!(
        //     "TRACE: set_experience_thumbnail_order {:?}",
        //     new_thumbnail_order
        // );

        let res = ureq::post(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/order",
            experience_id
        ))
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send_json(json!({ "thumbnailIds": new_thumbnail_order }));

        Self::handle_response(res)?;

        Ok(())
    }

    pub fn delete_experience_thumbnail(
        &mut self,
        experience_id: u64,
        thumbnail_id: u64,
    ) -> Result<(), String> {
        // println!("TRACE: delete_experience_thumbnail {}", thumbnail_id);

        let res = ureq::delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/thumbnails/{}",
            experience_id, thumbnail_id
        ))
        .set_auth(AuthType::CookieWithCsrfToken, &mut self.roblox_auth)?
        .send_string("");

        Self::handle_response(res)?;

        Ok(())
    }
}
