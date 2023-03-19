use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationResponse {
    pub id: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationsResponse {
    pub notification_string_configs: Vec<ListNotificationResponse>,
    pub previous_page_config: Option<String>,
    pub next_page_cursor: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationResponse {
    pub id: String,
    pub name: String,
    pub content: String,
}
