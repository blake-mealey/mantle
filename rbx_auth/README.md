# `rbx_auth`

Constructs a headers map and cookie jar that can be passed to a `reqwest` client to make authenticated
requests to Roblox APIs. Best used with the `rbx_api` crate.

## Usage

```rs
use rbx_auth::RobloxAuth;

let auth = RobloxAuth::new().await?;

let client = reqwest::Client::builder()
    .user_agent("Roblox/WinInet")
    .cookie_provider(Arc::new(auth.jar))
    .default_headers(auth.headers)
    .build();
```
