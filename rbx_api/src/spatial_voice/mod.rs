pub mod models;

use crate::{
    errors::RobloxApiResult, helpers::handle_as_json, models::AssetId,
    spatial_voice::models::UpdateSpatialVoiceSettingsResponse, RobloxApi,
};

use self::models::{GetSpatialVoiceSettingsResponse, UpdateSpatialVoiceSettingsRequest};

impl RobloxApi {
    pub async fn update_spatial_voice_settings(
        &self,
        experience_id: AssetId,
        settings: UpdateSpatialVoiceSettingsRequest,
    ) -> RobloxApiResult<UpdateSpatialVoiceSettingsResponse> {
        let req = self
            .client
            .post(&format!(
                "https://voice.roblox.com/v1/settings/universe/{}",
                experience_id
            ))
            .json(&settings);

        Ok(handle_as_json::<UpdateSpatialVoiceSettingsResponse>(req).await?)
    }

    pub async fn get_spatial_voice_settings(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<GetSpatialVoiceSettingsResponse> {
        let req = self.client.get(&format!(
            "https://voice.roblox.com/v1/settings/universe/{}",
            experience_id
        ));

        Ok(handle_as_json::<GetSpatialVoiceSettingsResponse>(req).await?)
    }
}
