use std::env;

use cookie::Cookie;

pub fn get() -> Result<String, String> {
    let cookie = match from_environment() {
        Some(v) => v,
        None => match from_roblox_studio() {
            Some(v) => v,
            None => return Err("Missing the ROBLOSECURITY environment variable".to_string()),
        },
    };

    Ok(Cookie::build(".ROBLOSECURITY", cookie)
        .domain(".roblox.com")
        .finish()
        .to_string())
}

fn from_environment() -> Option<String> {
    env::var("ROBLOSECURITY").ok()
}

#[cfg(target_os = "windows")]
fn from_roblox_studio() -> Option<String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey("SOFTWARE\\Roblox\\RobloxStudioBrowser\\roblox.com")
        .ok()?;
    let value: String = key.get_value(".ROBLOSECURITY").ok()?;

    parse_roblox_studio_cookie(&value)
}

#[cfg(target_os = "macos")]
fn from_roblox_studio() -> Option<String> {
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

    parse_roblox_studio_cookie(value)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn from_roblox_studio() -> Option<String> {
    None
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn parse_roblox_studio_cookie(value: &str) -> Option<String> {
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
