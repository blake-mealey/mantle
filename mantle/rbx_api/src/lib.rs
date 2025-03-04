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

pub struct RobloxApi {
    client: reqwest::Client,
    csrf_token_store: RobloxCsrfTokenStore,
}

impl RobloxApi {
    pub fn new(
        cookie_store: Arc<RobloxCookieStore>,
        csrf_token_store: RobloxCsrfTokenStore,
    ) -> RobloxApiResult<Self> {
        Ok(Self {
            csrf_token_store,
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Roblox/WinInet")
                .cookie_provider(cookie_store)
                .build()?,
        })
    }

    pub async fn validate_auth(&self) -> RobloxApiResult<()> {
        self.get_authenticated_user().await?;
        Ok(())
    }
}
