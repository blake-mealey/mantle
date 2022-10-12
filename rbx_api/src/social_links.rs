use serde_json::json;

use crate::{
    helpers::{handle, handle_as_json},
    models::{
        AssetId, CreateSocialLinkResponse, GetSocialLinkResponse, ListSocialLinksResponse,
        SocialLinkType,
    },
    RobloxApi, RobloxApiResult,
};

impl RobloxApi {
    pub async fn create_social_link(
        &self,
        experience_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> RobloxApiResult<CreateSocialLinkResponse> {
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links",
                experience_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        handle_as_json(req).await
    }

    pub async fn update_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
        title: String,
        url: String,
        link_type: SocialLinkType,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v1/universes/{}/social-links/{}",
                experience_id, social_link_id
            ))
            .json(&json!({
                "title": title,
                "url": url,
                "type": link_type,
            }));

        handle(req).await?;

        Ok(())
    }

    pub async fn delete_social_link(
        &self,
        experience_id: AssetId,
        social_link_id: AssetId,
    ) -> RobloxApiResult<()> {
        let req = self.client.delete(&format!(
            "https://develop.roblox.com/v1/universes/{}/social-links/{}",
            experience_id, social_link_id
        ));

        handle(req).await?;

        Ok(())
    }

    pub async fn list_social_links(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<GetSocialLinkResponse>> {
        let req = self.client.get(&format!(
            "https://games.roblox.com/v1/games/{}/social-links/list",
            experience_id
        ));

        Ok(handle_as_json::<ListSocialLinksResponse>(req).await?.data)
    }
}
