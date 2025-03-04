use serde::{Deserialize, Serialize};

use crate::models::{AssetId, SocialSlotType};

pub const DEFAULT_PLACE_NAME: &str = "Untitled Game";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaceResponse {
    pub place_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaceResponse {
    pub id: AssetId,
    pub current_saved_version: u64,
    pub name: String,
    pub description: String,
    pub max_player_count: u32,
    pub allow_copying: bool,
    pub social_slot_type: SocialSlotType,
    pub custom_social_slots_count: Option<u32>,
    pub is_root_place: bool,
}

impl From<GetPlaceResponse> for PlaceConfigurationModel {
    fn from(response: GetPlaceResponse) -> Self {
        PlaceConfigurationModel {
            name: response.name,
            description: response.description,
            max_player_count: response.max_player_count,
            allow_copying: response.allow_copying,
            social_slot_type: response.social_slot_type,
            custom_social_slots_count: response.custom_social_slots_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceConfigurationModel {
    pub name: String,
    pub description: String,
    pub max_player_count: u32,
    pub allow_copying: bool,
    pub social_slot_type: SocialSlotType,
    pub custom_social_slots_count: Option<u32>,
}

impl Default for PlaceConfigurationModel {
    fn default() -> Self {
        PlaceConfigurationModel {
            name: DEFAULT_PLACE_NAME.to_owned(),
            description: "Created with Mantle".to_owned(),
            max_player_count: 50,
            allow_copying: false,
            social_slot_type: SocialSlotType::Automatic,
            custom_social_slots_count: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlacesResponse {
    pub next_page_cursor: Option<String>,
    pub data: Vec<ListPlaceResponse>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaceResponse {
    pub id: AssetId,
}
