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
use std::{env::var, error::Error};
use traq::apis::{channel_api, configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let access_token = var("BOT_ACCESS_TOKEN")?;
    let conf = configuration::Configuration {
        bearer_access_token: Some(access_token),
        ..Default::default()
    };
    let channels = channel_api::get_channels(&conf, Some(true)).await?;
    println!("there are {} public channels", channels.public.len());
    Ok(())
}
```

## TLS Backend

This crate depends on [reqwest](https://crates.io/crates/reqwest) as an HTTPS client. As with reqwest, you can select TLS backend through this crate's feature flags.

- `native-tls`: the system-installed TLS backend
- `rustls-tls`: the TLS library written in Rust

`native-tls` is used by default. To use only `rustls-tls`, write dependencies as:

```toml
[dependencies]
traq.version = "..."
traq.default-features = false
traq.features = ["rustls-tls"]
```

For more information, see [docs in reqwest::tls](https://docs.rs/reqwest/0.12/reqwest/tls/index.html).
