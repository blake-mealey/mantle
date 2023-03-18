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
pub mod places;
pub mod social_links;
pub mod spatial_voice;
pub mod thumbnails;
pub mod notifications;

use errors::{RobloxApiError, RobloxApiResult};
use helpers::handle;
use rbx_auth::{RobloxAuth, WithRobloxAuth};

pub struct RobloxApi {
    client: reqwest::Client,
}

impl RobloxApi {
    pub fn new(roblox_auth: RobloxAuth) -> RobloxApiResult<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .connection_verbose(true)
                .user_agent("Roblox/WinInet")
                .roblox_auth(roblox_auth)
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
