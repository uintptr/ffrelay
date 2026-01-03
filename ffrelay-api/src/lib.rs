//! # ffrelay-api
//!
//! A Rust API client library for [Firefox Relay](https://relay.firefox.com),
//! Mozilla's email forwarding service that helps protect your privacy by creating
//! email aliases that forward to your real email address.
//!
//! ## Features
//!
//! - Create random or custom domain email aliases
//! - List all your email relays
//! - Delete email relays
//! - Retrieve profile information
//! - Support for both standard relays and domain relays
//!
//! ## Quick Start
//!
//! ```no_run
//! use ffrelay_api::api::FFRelayApi;
//! use ffrelay_api::types::FirefoxEmailRelayRequest;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the API client with your Firefox Relay token
//! let api = FFRelayApi::new("your-api-token-here");
//!
//! // Create a new random email relay
//! let request = FirefoxEmailRelayRequest::builder()
//!     .description("My new relay".to_string())
//!     .build();
//! let email = api.create(request).await?;
//! println!("Created relay: {}", email);
//!
//! // List all relays
//! let relays = api.list().await?;
//! for relay in relays {
//!     println!("{}: {}", relay.id, relay.full_address);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Getting Your API Token
//!
//! 1. Go to [Firefox Relay](https://relay.firefox.com)
//! 2. Sign in with your Firefox Account
//! 3. Navigate to the API settings to generate your token

pub mod api;
pub mod error;
pub mod types;
