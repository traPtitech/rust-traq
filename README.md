# rust-traq

[![Rust](https://github.com/H1rono/rust-traq/actions/workflows/rust.yml/badge.svg)](https://github.com/H1rono/rust-traq/actions/workflows/rust.yml)
[![Release](https://github.com/H1rono/rust-traq/actions/workflows/release.yml/badge.svg)](https://github.com/H1rono/rust-traq/actions/workflows/release.yml)
[![LICENSE](https://img.shields.io/github/license/H1rono/rust-traq)](https://github.com/H1rono/rust-traq/blob/main/LICENSE)
<!--
[![GitHub release](https://img.shields.io/github/v/release/H1rono/rust-traq?logo=github)](https://github.com/H1rono/rust-traq/releases/latest)
[![crate](https://img.shields.io/crates/v/traq?logo=rust)](https://crates.io/crates/traq)
-->

:warning: Community Driven :warning: This is a client library for the traQ API, written in Rust.

This crate is updated using [openapi-generator](https://openapi-generator.tech).

## Example

```rust
// `traq` will be the name of this crate
use traq::apis::{channel_api, configuration};

#[tokio::main]
async fn main() {
    let conf = configuration::Configuration {
        bearer_access_token: env::var("BOT_ACCESS_TOKEN").ok(),
        ..Default::default()
    };
    // #general
    let res = channel_api::get_channels(&conf, Some(true)).await;
    println!("{:?}", res);
}
```
