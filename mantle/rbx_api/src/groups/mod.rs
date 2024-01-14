pub mod models;

use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{handle, handle_as_json},
    models::{AssetId, Group, Id},
    RobloxApi,
};

use self::models::ListGroupRolesResponse;

impl RobloxApi {
    /// * `role_id` - Not the same as rank, must be retrieved using [`RobloxApi::list_group_roles`]
    pub async fn update_user_group_role<GroupId: Into<Id<Group>>>(
        &self,
        group_id: GroupId,
        user_id: AssetId,
        role_id: u64,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(format!(
                "https://groups.roblox.com/v1/groups/{}/users/{}",
                group_id.into(),
                user_id
            ))
            .json(&json!({ "roleId": role_id }));

        handle(req).await?;

        Ok(())
    }

    pub async fn list_group_roles<GroupId: Into<Id<Group>>>(
        &self,
        group_id: GroupId,
    ) -> RobloxApiResult<ListGroupRolesResponse> {
        let req = self.client.get(format!(
            "https://groups.roblox.com/v1/groups/{}/roles",
            group_id.into()
        ));

        handle_as_json(req).await
    }
}
