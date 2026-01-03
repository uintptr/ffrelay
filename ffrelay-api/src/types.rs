//! Data types for Firefox Relay API requests and responses.

use bon::Builder;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

/// Represents an email relay (alias) with its statistics and metadata.
///
/// This structure contains information about a single email relay,
/// including its unique identifier, email address, and usage statistics.
#[derive(Deserialize, Tabled)]
pub struct FirefoxEmailRelay {
    /// Unique identifier for this relay.
    pub id: u64,

    /// The full email address of the relay (e.g., "abc123@mozmail.com").
    pub full_address: String,

    /// User-provided description for this relay.
    pub description: String,

    /// Number of emails that have been blocked by this relay.
    pub num_blocked: u64,

    /// Number of emails that have been forwarded to your real email address.
    pub num_forwarded: u64,

    /// Number of emails you've replied to through this relay.
    pub num_replied: u64,

    /// Number of spam emails detected for this relay.
    pub num_spam: u64,
}

impl FirefoxEmailRelay {
    /// Checks if this relay is a custom domain relay.
    ///
    /// Returns `true` if this is a custom domain relay (requires premium subscription),
    /// or `false` if it's a standard @mozmail.com relay.
    ///
    /// # Example
    ///
    /// ```
    /// # use ffrelay_api::types::FirefoxEmailRelay;
    /// # use serde_json::json;
    /// # let relay: FirefoxEmailRelay = serde_json::from_value(json!({
    /// #     "id": 123,
    /// #     "full_address": "test@mozmail.com",
    /// #     "description": "test",
    /// #     "num_blocked": 0,
    /// #     "num_forwarded": 0,
    /// #     "num_replied": 0,
    /// #     "num_spam": 0
    /// # })).unwrap();
    /// assert_eq!(relay.is_domain(), false); // Standard relay
    /// ```
    pub fn is_domain(&self) -> bool {
        if let Some((_, dom)) = self.full_address.split_once('@') {
            !dom.eq("mozmail.com")
        } else {
            false
        }
    }
}

/// Request parameters for creating a new email relay.
///
/// Use the builder pattern to construct this request. The `description` field
/// is required, while `enabled` defaults to `true` and `address` is optional.
///
/// # Example
///
/// ```
/// use ffrelay_api::types::FirefoxEmailRelayRequest;
///
/// // Create a simple relay
/// let request = FirefoxEmailRelayRequest::builder()
///     .description("For newsletters".to_string())
///     .build();
///
/// // Create a custom domain relay (requires premium)
/// let request = FirefoxEmailRelayRequest::builder()
///     .description("Shopping sites".to_string())
///     .address("shopping".to_string())
///     .build();
/// ```
#[derive(Debug, Serialize, Builder)]
pub struct FirefoxEmailRelayRequest {
    /// Description for the relay to help you remember its purpose.
    description: String,

    /// Whether the relay should be enabled immediately (defaults to `true`).
    #[builder(default = true)]
    enabled: bool,

    /// Optional custom address for domain relays (requires premium subscription).
    /// If `None`, a random address will be generated.
    pub address: Option<String>,
}

/// Detailed information about a Firefox Relay profile.
///
/// Contains account-level information including subscription status,
/// usage statistics, privacy settings, and configuration options.
#[derive(Debug, Deserialize, Tabled)]
pub struct FirefoxRelayProfile {
    /// Unique identifier for this profile.
    pub id: u64,

    /// The API token for this profile (may be redacted in some responses).
    pub api_token: String,

    /// Whether the account has reached the maximum number of masks allowed.
    pub at_mask_limit: bool,

    /// URL to the user's avatar image.
    pub avatar: String,

    /// Date when the user subscribed to premium features (ISO 8601 format).
    pub date_subscribed: String,

    /// Total number of emails blocked across all relays.
    pub emails_blocked: u64,

    /// Total number of emails forwarded across all relays.
    pub emails_forwarded: u64,

    /// Total number of emails replied to across all relays.
    pub emails_replied: u64,

    /// Whether the account has the megabundle subscription.
    pub has_megabundle: bool,

    /// Whether the account has phone masking features.
    pub has_phone: bool,

    /// Whether the account has a premium subscription.
    pub has_premium: bool,

    /// Whether the account has Mozilla VPN.
    pub has_vpn: bool,

    /// Number of level one email trackers blocked.
    pub level_one_trackers_blocked: u64,

    /// Whether metrics collection is enabled for this profile.
    pub metrics_enabled: bool,

    /// Timestamp for the next allowed email creation attempt.
    pub next_email_try: String,

    /// Onboarding state for free tier users.
    pub onboarding_free_state: u32,

    /// General onboarding state.
    pub onboarding_state: u32,

    /// Whether level one email tracker removal is enabled.
    pub remove_level_one_email_trackers: bool,

    /// Whether server-side storage is enabled.
    pub server_storage: bool,

    /// Whether phone call logs are stored.
    pub store_phone_log: bool,

    /// Custom subdomain for premium users (e.g., "username" in username@mozilla.email).
    pub subdomain: String,

    /// Total number of email masks (relays) created.
    pub total_masks: u64,
}
