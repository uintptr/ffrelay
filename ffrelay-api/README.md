# ffrelay-api

Rust API client library for [Firefox Relay](https://relay.firefox.com), Mozilla's email forwarding service that helps protect your privacy.

## Features

- Create random or custom domain email aliases
- List all your email relays
- Delete email relays
- Retrieve profile information
- Support for both standard relays and domain relays

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ffrelay-api = "0.0.2"
```

## Usage

```rust
use ffrelay_api::api::FFRelayApi;
use ffrelay_api::types::FirefoxEmailRelayRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the API client with your Firefox Relay token
    let api = FFRelayApi::new("your-api-token-here");

    // Create a new random email relay
    let request = FirefoxEmailRelayRequest::builder()
        .description("My new relay")
        .build();
    let email = api.create(request).await?;
    println!("Created relay: {}", email);

    // List all relays
    let relays = api.list().await?;
    for relay in relays {
        println!("{}: {}", relay.id, relay.full_address);
    }

    // Delete a relay
    api.delete(relay_id).await?;

    Ok(())
}
```

## Getting Your API Token

1. Go to [Firefox Relay](https://relay.firefox.com)
2. Sign in with your Firefox Account
3. Navigate to the API settings to generate your token

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
