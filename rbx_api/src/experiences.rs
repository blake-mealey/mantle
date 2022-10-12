use reqwest::header;
use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{handle, handle_as_json},
    models::{
        AssetId, CreateExperienceResponse, ExperienceConfigurationModel, GetExperienceResponse,
    },
    RobloxApi,
};

impl RobloxApi {
    pub async fn create_experience(
        &self,
        group_id: Option<AssetId>,
    ) -> RobloxApiResult<CreateExperienceResponse> {
        let req = self
            .client
            .post("https://api.roblox.com/universes/create")
            .json(&json!({
                "templatePlaceIdToUse": 95206881,
                "groupId": group_id
            }));

        handle_as_json(req).await
    }

    pub async fn get_experience(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<GetExperienceResponse> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}",
            experience_id
        ));

        handle_as_json(req).await
    }

    pub async fn get_experience_configuration(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<ExperienceConfigurationModel> {
        let req = self.client.get(&format!(
            "https://develop.roblox.com/v1/universes/{}/configuration",
            experience_id
        ));

        handle_as_json(req).await
    }

    pub async fn configure_experience(
        &self,
        experience_id: AssetId,
        experience_configuration: &ExperienceConfigurationModel,
    ) -> RobloxApiResult<()> {
        let req = self
            .client
            .patch(&format!(
                "https://develop.roblox.com/v2/universes/{}/configuration",
                experience_id
            ))
            .json(experience_configuration);

        handle(req).await?;

        Ok(())
    }

    pub async fn set_experience_active(
        &self,
        experience_id: AssetId,
        active: bool,
    ) -> RobloxApiResult<()> {
        let endpoint = if active { "activate" } else { "deactivate" };
        let req = self
            .client
            .post(&format!(
                "https://develop.roblox.com/v1/universes/{}/{}",
                experience_id, endpoint
            ))
            .header(header::CONTENT_LENGTH, 0);

        handle(req).await?;

        Ok(())
    }
}
