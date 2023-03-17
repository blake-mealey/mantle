use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateNotificationResponse {
    pub id: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationsResponse {
    pub notificationStringConfigs: Vec<ListNotificationResponse>,
	pub previousPageCursor: String,
	pub nextPageCursor: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationResponse {
    pub id: String,
	pub name: String,
	pub content: String,
}
