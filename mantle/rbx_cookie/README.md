# `rbx_cookie`

Finds the `.ROBLOSECURITY` cookie from either a `ROBLOSECURITY` environment variable or the authenticated
local Roblox Studio installation (works for Windows and MacOS). Available as both a library and CLI.

## CLI

Install with `cargo install rbx_cookie`.

```sh
rbx_cookie --help
```

## Library

Disable default features to exclude the CLI dependencies with `cargo add rbx_cookie --no-default-features`, or
use the `default-features = false` configuration:

```toml
# Cargo.toml
[dependencies]
rbx_cookie = { version = "<version>", default-features = false }
```

```rs
// Returns the cookie as a formatted header ready to add to a request
let cookie = rbx_cookie::get();

// Returns the raw cookie value
let cookie = rbx_cookie::get_value();
```
