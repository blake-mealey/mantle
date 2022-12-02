---
sidebar_position: 2
title: Installation
---

There are three ways to install Mantle.

## Install with Foreman (recommended)

[Foreman](https://github.com/Roblox/foreman#readme) is a toolchain manager for Roblox tools. You can
configure Foreman to install Mantle with the following `foreman.toml` config:

```toml
[tools]
mantle = { source = "blake-mealey/mantle", version = "<version>" }
```

You can check for the latest Mantle version on the
[releases](https://github.com/blake-mealey/mantle/releases) page.

This is the recommended method as it enables consistent version management across your team,
provides better interop with other tools, and makes using Mantle in CI environments (especially
GitHub Action) simpler.

If you need help installing Foreman, check out the [Getting Started](/docs/getting-started#foreman)
guide.

## Install with Cargo

You can download and compile the [crate](https://crates.io/crates/rbx_mantle) from source with
Cargo:

```sh
cargo install rbx_mantle
```

## Manually download from releases (simplest)

You can download prebuilt binaries from the [latest
release](https://github.com/blake-mealey/mantle/releases).
