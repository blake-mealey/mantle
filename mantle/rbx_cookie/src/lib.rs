#[cfg(target_os = "windows")]
mod wincred;

#[cfg(target_os = "macos")]
mod binarycookies;

#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub mod wsl_windows;

use std::env;

use cookie::Cookie;
use log::{info, trace};

static COOKIE_NAME: &str = ".ROBLOSECURITY";

/// Returns the cookie as a formatted header ready to add to a request
pub fn get() -> Option<String> {
    let cookie = get_value()?;

    Some(
        Cookie::build(COOKIE_NAME, cookie)
            .domain(".roblox.com")
            .finish()
            .to_string(),
    )
}

/// Returns the raw cookie value
pub fn get_value() -> Option<String> {
    from_environment()
        .or_else(from_roblox_studio)
        .or_else(from_roblox_studio_legacy)
}

fn from_environment() -> Option<String> {
    trace!("Attempting to load cookie from ROBLOSECURITY environment variable.");
    match env::var("ROBLOSECURITY") {
        Ok(v) => {
            info!("Loaded cookie from ROBLOSECURITY environment variable.");
            Some(v)
        }
        Err(_) => None,
    }
}

#[cfg(target_os = "windows")]
fn from_roblox_studio() -> Option<String> {
    // First try the userid postfixed cookie
    trace!("Attempting to load cookie from Windows Credentials.");

    let user_id = wincred::get("https://www.roblox.com:RobloxStudioAuthuserid").ok();
    let cookie = user_id.as_ref().and_then(|user_id| {
        wincred::get(&format!(
            "https://www.roblox.com:RobloxStudioAuth{}{}",
            COOKIE_NAME, user_id
        ))
        .ok()
    });

    if cookie.is_some() {
        info!(
            "Loaded cookie from Windows Credentials (user_id: {}).",
            user_id.unwrap()
        );
        cookie
    } else {
        // Fallback to the old cookie
        trace!("Attempting to load cookie from Windows Credentials (legacy).");

        let cookie = wincred::get(&format!(
            "https://www.roblox.com:RobloxStudioAuth{}",
            COOKIE_NAME
        ))
        .ok()?;
        info!("Loaded cookie from Windows Credentials (legacy).");
        Some(cookie)
    }
}

#[cfg(target_os = "macos")]
fn from_roblox_studio() -> Option<String> {
    use std::fs;

    trace!("Attempting to load cookie from MacOS HTTPStorages.");

    let path = dirs::home_dir()?.join("Library/HTTPStorages/com.Roblox.RobloxStudio.binarycookies");

    let binary = fs::read(path).ok()?;

    let mut cookie_store = binarycookies::Cookies::new(false);
    cookie_store.parse_content(&binary).ok()?;

    if let Some(cookie) = cookie_store.find_by_name(COOKIE_NAME) {
        info!("Loaded cookie from MacOS HTTPStorages (single-user).");
        return Some(cookie.value.clone());
    }

    if let Some(user_id) = cookie_store.find_by_name("/RobloxStudioAuth/userid") {
        if let Some(cookie) = cookie_store.find_by_name(&format!(
            "/RobloxStudioAuth/.ROBLOSECURITY{}",
            user_id.value
        )) {
            info!(
                "Loaded cookie from MacOS HTTPStorages (user_id: {}).",
                user_id.value
            );
            return Some(cookie.value.clone());
        }
    }

    None
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn from_roblox_studio() -> Option<String> {
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    {
        if wsl_windows::is_wsl() {
            trace!("WSL detected, attempting to load cookie from Windows Credentials.");

            if let Some(cookie) = wsl_windows::get_roblosecurity_cookie() {
                info!("Loaded cookie from Windows Credentials via WSL.");
                return Some(cookie);
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn from_roblox_studio_legacy() -> Option<String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    trace!("Attempting to load cookie from Windows Registry.");

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey("SOFTWARE\\Roblox\\RobloxStudioBrowser\\roblox.com")
        .ok()?;
    let value: String = key.get_value(COOKIE_NAME).ok()?;

    if let Some(cookie) = parse_roblox_studio_cookie(&value) {
        info!("Loaded cookie from Windows Registry.");
        Some(cookie)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
fn from_roblox_studio_legacy() -> Option<String> {
    trace!("Attempting to load cookie from MacOS plist.");

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

    if let Some(cookie) = parse_roblox_studio_cookie(value) {
        info!("Loaded cookie from MacOS plist.");
        Some(cookie)
    } else {
        None
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn from_roblox_studio_legacy() -> Option<String> {
    #[cfg(all(target_os = "linux", not(target_os = "android")))]
    {
        if wsl_windows::is_wsl() {
            trace!("WSL detected, attempting to load cookie from Windows Registry via WSL.");

            let value = wsl_windows::read_registry(
                "SOFTWARE\\Roblox\\RobloxStudioBrowser\\roblox.com",
                COOKIE_NAME,
            )?;

            if let Some(cookie) = parse_roblox_studio_cookie(&value) {
                info!("Loaded cookie from Windows Registry via WSL.");
                return Some(cookie);
            }
        }
    }

    None
}

#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    all(target_os = "linux", not(target_os = "android"))
))]
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
