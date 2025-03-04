# `rbx_api`

Make requests to Roblox's web APIs. Currently does not support any Open Cloud APIs.

## Usage

```rs
use rbx_auth::{RobloxCookieStore, RobloxCsrfTokenStore};
use rbx_api::RobloxApi;

let cookie_store = Arc::new(RobloxCookieStore::new()?);
let csrf_token_store = RobloxCsrfTokenStore::new();
let api = RobloxApi::new(cookie_store, csrf_token_store)?;

let user = api.get_authenticated_user().await?;
```
