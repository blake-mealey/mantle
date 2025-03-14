pub mod asset_aliases;
pub mod asset_permissions;
pub mod assets;
pub mod badges;
pub mod developer_products;
pub mod errors;
pub mod experiences;
pub mod game_passes;
pub mod groups;
mod helpers;
pub mod models;
pub mod notifications;
pub mod places;
pub mod social_links;
pub mod spatial_voice;
pub mod thumbnails;
pub mod user;

use std::sync::Arc;

use errors::{RobloxApiError, RobloxApiResult};
use rbx_auth::{RobloxCookieStore, RobloxCsrfTokenStore};
use reqwest::header::{HeaderMap, HeaderValue};

pub struct RobloxApi {
    client: reqwest::Client,
    open_cloud_client: Option<reqwest::Client>,
    csrf_token_store: RobloxCsrfTokenStore,
}

impl RobloxApi {
    pub fn new(
        cookie_store: Arc<RobloxCookieStore>,
        csrf_token_store: RobloxCsrfTokenStore,
        open_cloud_api_key: Option<String>,
    ) -> RobloxApiResult<Self> {
        Ok(Self {
            csrf_token_store,
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Roblox/WinInet")
                .cookie_provider(cookie_store)
                .build()?,
            open_cloud_client: open_cloud_api_key
                .map(|api_key| {
                    let mut headers = HeaderMap::new();
                    headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());
                    println!("rbx_api/{}", env!("CARGO_PKG_VERSION"));
                    reqwest::Client::builder()
                        .connection_verbose(true)
                        .user_agent(format!("rbx_api/{}", env!("CARGO_PKG_VERSION")))
                        .default_headers(headers)
                        .build()
                })
                .map_or(Ok(None), |v| v.map(Some))?,
        })
    }

    pub async fn validate_auth(&self) -> RobloxApiResult<()> {
        self.get_authenticated_user().await?;
        Ok(())
    }
}
