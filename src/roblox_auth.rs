use std::env;

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

    pub fn get_api_key(self: &mut Self) -> Result<String, String> {
        if self.api_key.is_none() {
            let var = match env::var("ROBLOX_API_KEY") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            };
            self.api_key = Some(var);
        }
        match self.api_key.clone() {
            Some(api_key) => Ok(api_key),
            None => unreachable!("api_key must be set"),
        }
    }

    pub fn get_roblosecurity(self: &mut Self) -> Result<String, String> {
        if self.roblosecurity.is_none() {
            let var = match env::var("ROBLOSECURITY") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            };
            self.roblosecurity = Some(var);
        }
        match self.roblosecurity.clone() {
            Some(roblosecurity) => Ok(roblosecurity),
            None => unreachable!("roblosecurity must be set"),
        }
    }

    pub fn get_csrf_token(self: &mut Self) -> Result<String, String> {
        if self.csrf_token.is_none() {
            let res = ureq::post("https://auth.roblox.com")
                .set(
                    "cookie",
                    &format!(".ROBLOSECURITY={}", self.get_roblosecurity()?),
                )
                .send_string("");
            self.csrf_token = match res {
                Ok(_) => None,
                Err(ureq::Error::Status(_code, response)) => match response.status() {
                    403 => response.header("x-csrf-token").map(|v| v.to_owned()),
                    _ => None,
                },
                Err(e) => return Err(format!("{}", e)),
            };
        }

        match self.csrf_token.clone() {
            Some(csrf_token) => Ok(csrf_token),
            None => Ok("".to_owned()),
        }
    }
}
