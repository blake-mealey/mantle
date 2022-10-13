use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::models::AssetId;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSocialLinkResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSocialLinkResponse {
    pub id: AssetId,
    pub title: String,
    pub url: Url,
    #[serde(rename = "type")]
    pub link_type: SocialLinkType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum SocialLinkType {
    Facebook,
    Twitter,
    YouTube,
    Twitch,
    Discord,
    RobloxGroup,
    Guilded,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSocialLinksResponse {
    pub data: Vec<GetSocialLinkResponse>,
}
