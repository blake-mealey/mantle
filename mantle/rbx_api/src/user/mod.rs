use models::GetAuthenticatedUserResponse;

use crate::{errors::RobloxApiResult, helpers::handle_as_json, RobloxApi};

pub mod models;

impl RobloxApi {
    pub async fn get_authenticated_user(&self) -> RobloxApiResult<GetAuthenticatedUserResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .get("https://users.roblox.com/v1/users/authenticated"))
            })
            .await;

        handle_as_json(res).await
    }
}
