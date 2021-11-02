use serde::Deserialize;
use std::{clone::Clone, ffi::OsStr, fmt, fs, path::Path};

use crate::{
    commands::deploy::{ExperienceTemplateConfig, PlaceTemplateConfig},
    roblox_auth::RobloxAuth,
};

#[derive(Deserialize, Copy, Clone)]
pub enum DeployMode {
    Publish,
    Save,
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

#[derive(Deserialize)]
struct RobloxApiError {
    // There are some other possible properties but we currently have no use for them so they are not
    // included

    // Most error models have a `message` property
    message: Option<String>,

    // Some error models (500) have a `title` property instead
    title: Option<String>,

    // Some error models on older APIs have an errors array
    errors: Option<Vec<RobloxApiError>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaceManagementResponse {
    version_number: i32,
}

static INVALID_API_KEY_HELP: &str = "\
    Please check your ROBLOX_API_KEY environment variable. \n\
    \tIf you don't have an API key, you can create one at https://create.roblox.com/credentials \n\
    \tYou must ensure that your API key has enabled the 'Place Management API System' and you have \n\
    \tadded the place you are trying to upload to the API System Configuration. You also must ensure \n\
    \tthat your API key's IP whitelist includes the machine you are running this on. You can set it \n\
    \tto '0.0.0.0/0' to whitelist all IPs but this should only be used for testing purposes.";

fn get_roblox_api_error_message(response: ureq::Response) -> String {
    let is_json = response.content_type() == "application/json";

    fn get_message_from_error(error: RobloxApiError) -> Option<String> {
        return if error.message.is_some() {
            Some(error.message.unwrap())
        } else if error.title.is_some() {
            Some(error.title.unwrap())
        } else if error.errors.is_some() {
            for e in error.errors.unwrap() {
                if let Some(message) = get_message_from_error(e) {
                    return Some(message);
                }
            }
            None
        } else {
            None
        };
    }

    let result: Option<String> = if is_json {
        match response.into_json::<RobloxApiError>() {
            Ok(v) => get_message_from_error(v),
            Err(_) => None,
        }
    } else {
        response.into_string().ok()
    };

    result.unwrap_or_else(|| "Unknown error".to_string())
}

pub struct UploadResult {
    pub place_version: i32,
}

pub struct RobloxApi {
    roblox_auth: RobloxAuth,
}

impl RobloxApi {
    pub fn new(roblox_auth: RobloxAuth) -> Self {
        Self { roblox_auth }
    }

    pub fn upload_place(
        self: &mut Self,
        project_file: &str,
        experience_id: u64,
        place_id: u64,
        mode: DeployMode,
    ) -> Result<UploadResult, String> {
        let api_key = &match self.roblox_auth.get_api_key() {
            Ok(val) => val,
            Err(_) => return Err(INVALID_API_KEY_HELP.to_string()),
        };

        let project_type = match Path::new(project_file).extension().and_then(OsStr::to_str) {
            Some("rbxlx") => ProjectType::Xml,
            Some("rbxl") => ProjectType::Binary,
            Some(v) => return Err(format!("Invalid project file extension: {}", v)),
            None => {
                return Err(format!(
                    "No project file extension in project file: {}",
                    project_file
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
        .set("x-api-key", api_key)
        .set("Content-Type", content_type)
        .query("versionType", version_type);

        let res = match project_type {
            ProjectType::Xml => {
                let data = match fs::read_to_string(project_file) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Unable to read project file: {}\n\t{}",
                            project_file, e
                        ))
                    }
                };
                println!("\t📦 Uploading file: {}", project_file);
                req.send_string(&data)
            }
            ProjectType::Binary => {
                let data = match fs::read(project_file) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!(
                            "Unable to read project file: {}\n\t{}",
                            project_file, e
                        ))
                    }
                };
                println!("🚀 Uploading file: {}", project_file);
                req.send_bytes(&data)
            }
        };

        match res {
            Ok(response) => {
                let model = response.into_json::<PlaceManagementResponse>().unwrap();
                println!(
                    "\
                \t🎉 Successfully {} to Roblox! \n\
                \t\tView place at https://www.roblox.com/games/{} \n\
                \t\tVersion Number: {}",
                    version_type.to_lowercase(),
                    place_id,
                    model.version_number
                );
                Ok(UploadResult {
                    place_version: model.version_number,
                })
            }
            Err(ureq::Error::Status(_code, response)) => {
                match (response.status(), get_roblox_api_error_message(response)) {
                    (400, message) => Err(format!("Invalid request or file content: {}", message)),
                    (401, message) => Err(format!(
                        "API key not valid for operation: {}\n   {}",
                        message, INVALID_API_KEY_HELP
                    )),
                    (403, message) => Err(format!("Publish not allowed on place: {}", message)),
                    (404, message) => Err(format!("Place or universe does not exist: {}", message)),
                    (409, message) => Err(format!("Place not part of the universe: {}", message)),
                    (500, message) => Err(format!("Server internal error: {}", message)),
                    (status, message) => {
                        Err(format!("Unknown error (status {}): {}", status, message))
                    }
                }
            }
            Err(e) => Err(format!("Unknown error: {}", e)),
        }
    }

    pub fn configure_experience(
        self: &mut Self,
        experience_id: u64,
        experience_template: &ExperienceTemplateConfig,
    ) -> Result<(), String> {
        let roblosecurity = match self.roblox_auth.get_roblosecurity() {
            Ok(val) => val,
            Err(_) => {
                return Err("Please check your ROBLOSECURITY environment variable.".to_owned())
            }
        };

        let csrf_token = match self.roblox_auth.get_csrf_token() {
            Ok(val) => val,
            Err(e) => return Err(format!("Failed to get the CSRF token\n\t{}", e)),
        };

        let json_data = match serde_json::to_value(&experience_template) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to serialize experience template\n\t{}", e)),
        };

        let res = ureq::request(
            "PATCH",
            &format!(
                "https://develop.roblox.com/v2/universes/{}/configuration",
                experience_id
            ),
        )
        .set("cookie", &format!(".ROBLOSECURITY={}", roblosecurity))
        .set("x-csrf-token", &csrf_token)
        .set("Content-Type", "application/json")
        .send_json(json_data);

        match res {
            Ok(_) => return Ok(()),
            Err(ureq::Error::Status(_code, response)) => {
                match (response.status(), get_roblox_api_error_message(response)) {
                    (status, message) => {
                        Err(format!("Unknown error (status {}): {}", status, message))
                    }
                }
            }
            Err(e) => Err(format!("Unknown error: {}", e)),
        }
    }

    pub fn configure_place(
        self: &mut Self,
        place_id: u64,
        place_template: &PlaceTemplateConfig,
    ) -> Result<(), String> {
        let roblosecurity = match self.roblox_auth.get_roblosecurity() {
            Ok(val) => val,
            Err(_) => return Err("Please check your ROBLOSECURITY environment variable".to_owned()),
        };

        let csrf_token = match self.roblox_auth.get_csrf_token() {
            Ok(val) => val,
            Err(e) => return Err(format!("Failed to get the CSRF token\n\t{}", e)),
        };

        let json_data = match serde_json::to_value(&place_template) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to serialize place template\n\t{}", e)),
        };

        let res = ureq::request(
            "PATCH",
            &format!("https://develop.roblox.com/v2/places/{}", place_id),
        )
        .set("cookie", &format!(".ROBLOSECURITY={}", roblosecurity))
        .set("x-csrf-token", &csrf_token)
        .set("Content-Type", "application/json")
        .send_json(json_data);

        match res {
            Ok(_) => return Ok(()),
            Err(ureq::Error::Status(_code, response)) => {
                match (response.status(), get_roblox_api_error_message(response)) {
                    (status, message) => {
                        Err(format!("Unknown error (status {}): {}", status, message))
                    }
                }
            }
            Err(e) => Err(format!("Unknown error: {}", e)),
        }
    }
}
