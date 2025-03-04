# `rbx_auth`

Helpers for working with legacy Roblox authentication (`.ROBLOSECURITY` cookies and `X-Csrf-Token` headers).
Best used with the `rbx_api` crate. Available as both a library and CLI.

## CLI

Install with `cargo install rbx_auth`.

```sh
rbx_auth --help
```

## Library

Disable default features to exclude the CLI dependencies with `cargo add rbx_auth --no-default-features`, or
use the `default-features = false` configuration:

```toml
# Cargo.toml
[dependencies]
rbx_auth = { version = "<version>", default-features = false }
```

```rs
use rbx_auth::{RobloxCookieStore, RobloxCsrfTokenStore};

let cookie_store = Arc::new(RobloxCookieStore::new()?);
let csrf_token_store = RobloxCsrfTokenStore::new();

let client = reqwest::Client::builder()
    .user_agent("Roblox/WinInet")
    .cookie_provider(cookie_store)
    .build()?;

let res = csrf_token_store
    .send_request(|| async {
        Ok(client.get("https://users.roblox.com/v1/users/authenticated"))
    })
    .await?;
```
