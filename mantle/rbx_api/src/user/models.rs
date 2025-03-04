use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthenticatedUserResponse {
    pub id: AssetId,
    pub name: String,
    pub display_name: String,
}
