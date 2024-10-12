use std::sync::Arc;

use reqwest::{
    cookie::Jar,
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum RobloxAuthError {
    #[error("HTTP client error.")]
    HttpClient(#[from] reqwest::Error),
    #[error("Unable to find ROBLOSECURITY cookie. Login to Roblox Studio or set the ROBLOSECURITY environment variable.")]
    MissingRoblosecurityCookie,
    #[error("Request for CSRF token did not return an X-CSRF-Token header.")]
    MissingCsrfToken,
}

// Temporary to make the new errors backwards compatible with the String errors throughout the project.
impl From<RobloxAuthError> for String {
    fn from(e: RobloxAuthError) -> Self {
        e.to_string()
    }
}

#[derive(Debug)]
pub struct RobloxAuth {
    pub jar: Jar,
    pub headers: HeaderMap,
}

impl RobloxAuth {
    pub async fn new() -> Result<Self, RobloxAuthError> {
        let roblosecurity_cookie =
            rbx_cookie::get().ok_or(RobloxAuthError::MissingRoblosecurityCookie)?;

        let jar = Jar::default();
        let url = "https://roblox.com".parse::<Url>().unwrap();
        jar.add_cookie_str(&roblosecurity_cookie, &url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-CSRF-Token", get_csrf_token(&roblosecurity_cookie).await?);

        Ok(Self { jar, headers })
    }
}

async fn get_csrf_token(roblosecurity_cookie: &str) -> Result<HeaderValue, RobloxAuthError> {
    let response = Client::new()
        .post("https://auth.roblox.com//")
        .header(header::COOKIE, roblosecurity_cookie)
        .header(header::CONTENT_LENGTH, 0)
        .send()
        .await?;

    response
        .headers()
        .get("X-CSRF-Token")
        .map(|v| v.to_owned())
        .ok_or(RobloxAuthError::MissingCsrfToken)
}

pub trait WithRobloxAuth {
    fn roblox_auth(self, roblox_auth: RobloxAuth) -> Self;
}

impl WithRobloxAuth for ClientBuilder {
    fn roblox_auth(self, roblox_auth: RobloxAuth) -> Self {
        self.cookie_provider(Arc::new(roblox_auth.jar))
            .default_headers(roblox_auth.headers)
    }
}
