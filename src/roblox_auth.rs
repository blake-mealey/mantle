use std::env;

use ureq::Cookie;

use crate::roblox_api::INVALID_API_KEY_HELP;

pub enum AuthType {
    ApiKey,
    Cookie,
    CookieAndCsrfToken,
}

pub trait RequestExt {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String>;
}

impl RequestExt for ureq::Request {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String> {
        match auth_type {
            AuthType::ApiKey => Ok(self.set("x-api-key", &auth.get_api_key()?)),
            AuthType::Cookie => Ok(self.set("cookie", &auth.get_roblosecurity_cookie()?)),
            AuthType::CookieAndCsrfToken => Ok(self
                .set("cookie", &auth.get_roblosecurity_cookie()?)
                .set("x-csrf-token", &auth.get_csrf_token()?)),
        }
    }
}

#[derive(Default)]
pub struct RobloxAuth {
    api_key: Option<String>,
    roblosecurity: Option<String>,
    csrf_token: Option<String>,
}

impl RobloxAuth {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_api_key(&mut self) -> Result<String, String> {
        if self.api_key.is_none() {
            let var = match env::var("ROBLOX_API_KEY") {
                Ok(v) => v,
                Err(_) => return Err(INVALID_API_KEY_HELP.to_owned()),
            };
            self.api_key = Some(var);
        }
        Ok(self.api_key.clone().unwrap())
    }

    pub fn get_roblosecurity(&mut self) -> Result<String, String> {
        if self.roblosecurity.is_none() {
            let var = match env::var("ROBLOSECURITY") {
                Ok(v) => v,
                Err(_) => {
                    return Err("Please check your ROBLOSECURITY environment variable".to_owned())
                }
            };
            self.roblosecurity = Some(var);
        }
        Ok(self.roblosecurity.clone().unwrap())
    }

    pub fn get_roblosecurity_cookie(&mut self) -> Result<String, String> {
        Ok(Cookie::new(".ROBLOSECURITY", self.get_roblosecurity()?).to_string())
    }

    pub fn get_csrf_token(&mut self) -> Result<String, String> {
        if self.csrf_token.is_none() {
            let res = ureq::post("https://auth.roblox.com")
                .set_auth(AuthType::Cookie, self)?
                .send_string("");
            self.csrf_token = match res {
                Ok(_) => {
                    return Err("Request for csrf token returned 200 (expected 403)".to_owned())
                }
                Err(ureq::Error::Status(_code, response)) => match response.status() {
                    403 => Some(
                        response
                            .header("x-csrf-token")
                            .map(|v| v.to_owned())
                            .ok_or(
                                "Request for csrf token did not return an x-csrf-token header"
                                    .to_owned(),
                            )?,
                    ),
                    status => {
                        return Err(format!(
                            "Request for csrf token returned {} (expected 403)",
                            status
                        ))
                    }
                },
                Err(e) => return Err(format!("Request for csrf token failed: {}", e)),
            };
        }
        Ok(self.csrf_token.clone().unwrap())
    }
}
