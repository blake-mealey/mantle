use std::{
    future::Future,
    ops::{Deref, DerefMut},
    sync::Arc,
    thread::sleep,
    time::{Duration, SystemTime},
};

use rbx_auth::{RobloxAuth, WithRobloxAuth};
use reqwest::{header, Client, ClientBuilder, Request, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = RobloxAuth::new().await?;

    let rbx_client = RobloxReqwestClient::new(auth).unwrap();
    rbx_client.execute(rbx_client.get("asdf").build()?).await?;

    let client = reqwest::Client::builder()
        .user_agent("Roblox/WinInet")
        .roblox_auth(auth)
        .build()?;

    let start = SystemTime::now();

    loop {
        let res = client
            .post("https://auth.roblox.com")
            .header(header::CONTENT_LENGTH, 0)
            .send()
            .await?;

        if !res.status().is_success() {
            println!(
                "Received {} after: {}s",
                res.status().to_string(),
                start.elapsed()?.as_secs()
            );
            break;
        }

        println!(
            "Received {} after: {}s",
            res.status().to_string(),
            start.elapsed()?.as_secs()
        );

        sleep(Duration::new(2, 0))
    }

    Ok(())
}

struct RobloxReqwestClient {
    http_client: Option<reqwest::Client>,
    auth: RobloxAuth,
    create_builder: Box<dyn FnMut() -> ClientBuilder>,
}

impl Deref for RobloxReqwestClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        self.client().unwrap()
    }
}

// impl DerefMut for RobloxReqwestClient {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.client().unwrap()
//     }
// }

impl RobloxReqwestClient {
    pub fn new_with_builder(
        auth: RobloxAuth,
        create_builder: impl FnMut() -> ClientBuilder + 'static,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            create_builder: Box::new(create_builder),
            auth,
            http_client: None,
        })
    }

    pub fn new(auth: RobloxAuth) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_builder(auth, || Client::builder())
    }

    fn construct_client(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.http_client = Some(
            (self.create_builder)()
                .user_agent("Roblox/WinInet")
                .cookie_provider(Arc::new(self.auth.jar))
                .default_headers(self.auth.headers)
                .build()?,
        );

        Ok(())
    }

    fn client(&mut self) -> Result<&Client, Box<dyn std::error::Error>> {
        if self.http_client.is_none() {
            self.construct_client();
        }

        Ok(self.http_client.as_ref().unwrap())
    }

    pub async fn execute(
        &mut self,
        request: Request,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let cloned_request = request.try_clone().unwrap();
        let mut response = self.client()?.execute(request).await?;

        if matches!(response.status(), StatusCode::FORBIDDEN) {
            self.auth.refresh(response.headers())?;
            self.construct_client()?;

            response = self.client()?.execute(cloned_request).await?;
        }

        Ok(response)
    }
}
