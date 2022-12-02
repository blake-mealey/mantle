# `rbx_api`

Make requests to Roblox's web APIs. Currently does not support any Open Cloud APIs.

## Usage

```rs
use rbx_auth::RobloxAuth;
use rbx_api::RobloxApi;

let auth = RobloxAuth::new().await?;
let api = RobloxApi::new(auth)?;

api.upload_place("MyPlace.rbxl".into(), 123456)?;
```
