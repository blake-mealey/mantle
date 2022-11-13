use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpatialVoiceSettingsRequest {
    pub opt_in: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSpatialVoiceSettingsResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSpatialVoiceSettingsResponse {
    pub is_universe_enabled_for_voice: bool,
}
