# rust-traq

[![Rust](https://github.com/traPtitech/rust-traq/actions/workflows/rust.yml/badge.svg)](https://github.com/traPtitech/rust-traq/actions/workflows/rust.yml)
[![Release](https://github.com/traPtitech/rust-traq/actions/workflows/release.yml/badge.svg)](https://github.com/traPtitech/rust-traq/actions/workflows/release.yml)
[![LICENSE](https://img.shields.io/github/license/traPtitech/rust-traq)](https://github.com/traPtitech/rust-traq/blob/main/LICENSE)

[![GitHub release](https://img.shields.io/github/v/release/traPtitech/rust-traq?logo=github)](https://github.com/traPtitech/rust-traq/releases/latest)
[![crate](https://img.shields.io/crates/v/traq?logo=rust)](https://crates.io/crates/traq)

⚠️ Community Driven ⚠️ This is a client library for the traQ API, written in Rust.

This crate is updated using [openapi-generator](https://openapi-generator.tech).

## Example

Add this crate using `cargo add traq`, then write in `main.rs`:

```rust
use traq::apis::{channel_api, configuration};

#[tokio::main]
async fn main() {
    let conf = configuration::Configuration {
        bearer_access_token: env::var("BOT_ACCESS_TOKEN").ok(),
        ..Default::default()
    };
    let res = channel_api::get_channels(&conf, Some(true)).await;
    println!("{:?}", res);
}
```
