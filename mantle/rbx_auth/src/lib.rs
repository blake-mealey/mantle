use std::sync::Arc;

use log::debug;
use reqwest::{cookie::Jar, header::HeaderValue, ClientBuilder, StatusCode};
use reqwest_chain::{ChainMiddleware, Chainer};
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
}

impl RobloxAuth {
    pub async fn new() -> Result<Self, RobloxAuthError> {
        let roblosecurity_cookie =
            rbx_cookie::get().ok_or(RobloxAuthError::MissingRoblosecurityCookie)?;

        let jar = Jar::default();
        let url = "https://roblox.com".parse::<Url>().unwrap();
        jar.add_cookie_str(&roblosecurity_cookie, &url);

        Ok(Self { jar })
    }
}

pub trait WithRobloxAuth {
    fn roblox_auth(
        self,
        roblox_auth: RobloxAuth,
    ) -> Result<reqwest_middleware::ClientBuilder, reqwest::Error>;
}

impl WithRobloxAuth for ClientBuilder {
    fn roblox_auth(
        self,
        roblox_auth: RobloxAuth,
    ) -> Result<reqwest_middleware::ClientBuilder, reqwest::Error> {
        let reqwest_client = self.cookie_provider(Arc::new(roblox_auth.jar)).build()?;
        Ok(reqwest_middleware::ClientBuilder::new(reqwest_client)
            .with(ChainMiddleware::new(CsrfTokenMiddleware)))
    }
}

struct CsrfTokenMiddleware;

#[async_trait::async_trait]
impl Chainer for CsrfTokenMiddleware {
    // TODO: is this state per request chain? Maybe we should store this in the CsrfTokenMiddleware struct instead
    type State = Option<HeaderValue>;

    async fn chain(
        &self,
        result: Result<reqwest::Response, reqwest_middleware::Error>,
        state: &mut Self::State,
        request: &mut reqwest::Request,
    ) -> Result<Option<reqwest::Response>, reqwest_middleware::Error> {
        let response = result?;

        let csrf_token = response.headers().get("X-CSRF-TOKEN").cloned();
        if let Some(value) = csrf_token {
            debug!(
                "Store X-CSRF-Token: {}",
                value.to_str().unwrap_or("INVALID")
            );
            *state = Some(value)
        }

        match response.status() {
            StatusCode::FORBIDDEN => match state {
                Some(value) => {
                    debug!(
                        "Retry with X-CSRF-Token: {}",
                        value.to_str().unwrap_or("INVALID")
                    );
                    request.headers_mut().insert("X-CSRF-TOKEN", value.clone());
                    Ok(None)
                }
                None => Ok(Some(response)),
            },
            _ => Ok(Some(response)),
        }
    }
}
