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
        let roblosecurity_cookie = rbx_cookie::get()
            .ok_or_else(|| "Unable to find ROBLOSECURITY cookie. Login to Roblox Studio or set the ROBLOSECURITY environment variable".to_owned())?;

        let jar = Jar::default();
        let url = "https://roblox.com".parse::<Url>().unwrap();
        jar.add_cookie_str(&roblosecurity_cookie, &url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-CSRF-Token", get_csrf_token(&roblosecurity_cookie).await?);

        Ok(Self { jar, headers })
    }
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
