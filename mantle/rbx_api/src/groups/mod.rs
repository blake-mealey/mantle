pub mod models;

use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{handle, handle_as_json},
    models::{GroupId, UserId},
    RobloxApi,
};

use self::models::ListRolesResponse;

impl RobloxApi {
    /// * `role_id` - Not the same as rank, must be retrieved using [`RobloxApi::get_all_roles`]
    pub async fn update_user_role(
        &self,
        group_id: GroupId,
        user_id: UserId,
        role_id: u64,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://groups.roblox.com/v1/groups/{}/users/{}",
                group_id, user_id
            ))
            .json(&json!({ "roleId": role_id }));

        handle(req).await?;

        Ok(())
    }

    pub async fn get_all_roles(&self, group_id: GroupId) -> RobloxApiResult<ListRolesResponse> {
        let req = self.client.get(&format!(
            "https://groups.roblox.com/v1/groups/{}/roles",
            group_id
        ));

        handle_as_json(req).await
    }
}
