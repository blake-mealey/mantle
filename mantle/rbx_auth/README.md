# `rbx_auth`

Constructs a headers map and cookie jar that can be passed to a `reqwest` client to make authenticated
requests to Roblox APIs. Best used with the `rbx_api` crate. Available as both a library and CLI.

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
use rbx_auth::{RobloxAuth, WithRobloxAuth};

let auth = RobloxAuth::new().await?;

let client = reqwest::Client::builder()
    .user_agent("Roblox/WinInet")
    .roblox_auth(auth)
    .build()?;
```
