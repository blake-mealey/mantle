use std::env;

use ureq::Cookie;

pub enum AuthType {
    Cookie,
    CookieAndCsrfToken,
    CookieAndCsrfTokenAndVerificationToken { verification_token: String },
}

pub trait RequestExt {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String>;
}

impl RequestExt for ureq::Request {
    fn set_auth(self, auth_type: AuthType, auth: &mut RobloxAuth) -> Result<ureq::Request, String> {
        match auth_type {
            AuthType::Cookie => Ok(self.set("cookie", &auth.get_roblosecurity_cookie()?)),
            AuthType::CookieAndCsrfToken => Ok(self
                .set("cookie", &auth.get_roblosecurity_cookie()?)
                .set("x-csrf-token", &auth.get_csrf_token()?)),
            AuthType::CookieAndCsrfTokenAndVerificationToken { verification_token } => Ok(self
                .set(
                    "cookie",
                    &format!(
                        "{}; {}",
                        auth.get_roblosecurity_cookie()?,
                        Cookie::new("__RequestVerificationToken", verification_token).to_string()
                    ),
                )
                .set("x-csrf-token", &auth.get_csrf_token()?)),
        }
    }
}

#[derive(Default)]
pub struct RobloxAuth {
    roblosecurity: Option<String>,
    csrf_token: Option<String>,
}

impl RobloxAuth {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_roblosecurity(&mut self) -> Result<String, String> {
        if self.roblosecurity.is_none() {
            self.roblosecurity = match env::var("ROBLOSECURITY").ok() {
                Some(v) => Some(v),
                None => get_roblosecurity_from_roblox_studio(),
            };
            if self.roblosecurity.is_none() {
                return Err("Missing the ROBLOSECURITY environment variable".to_string());
            }
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
                            .ok_or_else(|| {
                                "Request for csrf token did not return an x-csrf-token header"
                                    .to_owned()
                            })?,
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

#[cfg(windows)]
fn get_roblosecurity_from_roblox_studio() -> Option<String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey("SOFTWARE\\Roblox\\RobloxStudioBrowser\\roblox.com")
        .ok()?;
    let value: String = key.get_value(".ROBLOSECURITY").ok()?;

    for item in value.split(',') {
        let parts = item.split("::").collect::<Vec<_>>();
        match &parts[..] {
            ["COOK", cookie] => {
                if !cookie.starts_with('<') || !cookie.ends_with('>') {
                    return None;
                }
                return Some(cookie[1..cookie.len() - 1].to_owned());
            }
            _ => continue,
        }
    }

    None
}

#[cfg(not(windows))]
fn get_roblosecurity_from_roblox_studio() -> Option<String> {
    None
}
