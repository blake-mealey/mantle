use thiserror::Error;

use crate::models::AssetTypeId;

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
