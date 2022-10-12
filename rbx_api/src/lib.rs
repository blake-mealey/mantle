pub mod asset_aliases;
pub mod assets;
pub mod badges;
pub mod developer_products;
pub mod experiences;
pub mod game_passes;
mod helpers;
pub mod models;
pub mod places;
pub mod social_links;
pub mod thumbnails;

use std::sync::Arc;

use helpers::handle;
use rbx_auth::RobloxAuth;
use thiserror::Error;

use models::*;

// TODO: Improve some of these error messages.
#[derive(Error, Debug)]
pub enum RobloxApiError {
    #[error("HTTP client error.")]
    HttpClient(#[from] reqwest::Error),
    #[error("Authorization has been denied for this request. Check your ROBLOSECURITY cookie.")]
    Authorization,
    #[error("Roblox error: {0}")]
    Roblox(String),
    #[error("Failed to parse JSON response.")]
    ParseJson(#[from] serde_json::Error),
    #[error("Failed to parse HTML response.")]
    ParseHtml,
    #[error("Failed to parse AssetId.")]
    ParseAssetId,
    #[error("Failed to read file.")]
    ReadFile(#[from] std::io::Error),
    #[error("Failed to determine file name for path {0}.")]
    NoFileName(String),
    #[error("Invalid file extension for path {0}.")]
    InvalidFileExtension(String),
    #[error("Failed to read utf8 data.")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("No create quotas found for asset type {0}")]
    MissingCreateQuota(AssetTypeId),
}

// Temporary to make the new errors backwards compatible with the String errors throughout the project.
impl From<RobloxApiError> for String {
    fn from(e: RobloxApiError) -> Self {
        e.to_string()
    }
}

pub type RobloxApiResult<T> = Result<T, RobloxApiError>;

pub struct RobloxApi {
    client: reqwest::Client,
}

impl RobloxApi {
    pub fn new(roblox_auth: RobloxAuth) -> RobloxApiResult<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Roblox/WinInet")
                .cookie_provider(Arc::new(roblox_auth.jar))
                .default_headers(roblox_auth.headers)
                .build()?,
        })
    }

    pub async fn validate_auth(&self) -> RobloxApiResult<()> {
        let req = self
            .client
            .get("https://users.roblox.com/v1/users/authenticated");

        handle(req)
            .await
            .map_err(|_| RobloxApiError::Authorization)?;

        Ok(())
    }
}
