use models::GetAuthenticatedUserResponse;

use crate::{errors::RobloxApiResult, helpers::handle_as_json, RobloxApi};

pub mod models;

impl RobloxApi {
    pub async fn get_authenticated_user(&self) -> RobloxApiResult<GetAuthenticatedUserResponse> {
        let req = self
            .client
            .get("https://users.roblox.com/v1/users/authenticated");

        handle_as_json(req).await
    }
}
