use std::env;

use cookie::Cookie;
use reqwest::{
    cookie::Jar,
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use url::Url;

pub struct RobloxAuth {
    pub jar: Jar,
    pub headers: HeaderMap,
}

impl RobloxAuth {
    pub async fn new() -> Result<Self, String> {
        let roblosecurity_cookie = get_roblosecurity_cookie()?;

        let jar = Jar::default();
        let url = "https://roblox.com".parse::<Url>().unwrap();
        jar.add_cookie_str(&roblosecurity_cookie, &url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-CSRF-Token", get_csrf_token(&roblosecurity_cookie).await?);

        Ok(Self { jar, headers })
    }
}

fn get_roblosecurity_cookie() -> Result<String, String> {
    let roblosecurity = match get_roblosecurity_from_environment() {
        Some(v) => v,
        None => match get_roblosecurity_from_roblox_studio() {
            Some(v) => v,
            None => return Err("Missing the ROBLOSECURITY environment variable".to_string()),
        },
    };

    Ok(Cookie::build(".ROBLOSECURITY", roblosecurity)
        .domain(".roblox.com")
        .finish()
        .to_string())
}

fn get_roblosecurity_from_environment() -> Option<String> {
    env::var("ROBLOSECURITY").ok()
}

fn parse_roblosecurity_from_roblox_studio(value: &str) -> Option<String> {
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

#[cfg(target_os = "windows")]
fn get_roblosecurity_from_roblox_studio() -> Option<String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey("SOFTWARE\\Roblox\\RobloxStudioBrowser\\roblox.com")
        .ok()?;
    let value: String = key.get_value(".ROBLOSECURITY").ok()?;

    parse_roblosecurity_from_roblox_studio(value)
}

#[cfg(target_os = "macos")]
fn get_roblosecurity_from_roblox_studio() -> Option<String> {
    let path = dirs::home_dir()?.join("Library/Preferences/com.roblox.RobloxStudioBrowser.plist");
    let list = plist::Value::from_file(path).ok()?;

    let value = list
        .as_dictionary()
        .and_then(|dict| {
            dict.into_iter().find_map(|(key, value)| {
                if key.ends_with("ROBLOSECURITY") {
                    Some(value)
                } else {
                    None
                }
            })
        })?
        .as_string()?;

    parse_roblosecurity_from_roblox_studio(value)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_roblosecurity_from_roblox_studio() -> Option<String> {
    None
}

async fn get_csrf_token(roblosecurity_cookie: &str) -> Result<HeaderValue, String> {
    let res = Client::new()
        .post("https://auth.roblox.com")
        .header(header::COOKIE, roblosecurity_cookie)
        .header(header::CONTENT_LENGTH, 0)
        .send()
        .await;
    match res {
        Ok(response) => {
            let status_code = response.status();
            if status_code == 403 {
                response
                    .headers()
                    .get("X-CSRF-Token")
                    .map(|v| v.to_owned())
                    .ok_or_else(|| {
                        "Request for CSRF token did not return an X-CSRF-Token header".to_owned()
                    })
            } else {
                Err(format!(
                    "Request for CSRF token returned {} (expected 403)",
                    status_code
                ))
            }
        }
        Err(error) => return Err(format!("Request for CSRF token failed: {}", error)),
    }
}
