//! Firefox Relay API client implementation.

use log::info;
use reqwest::Client;

use crate::{
    error::{Error, Result},
    types::{FirefoxEmailRelay, FirefoxEmailRelayRequest, FirefoxRelayProfile},
};

/// The main API client for interacting with Firefox Relay.
///
/// This struct provides methods to create, list, and delete email relays,
/// as well as retrieve profile information.
///
/// # Example
///
/// ```no_run
/// use ffrelay_api::api::FFRelayApi;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let api = FFRelayApi::new("your-api-token");
/// let relays = api.list().await?;
/// # Ok(())
/// # }
/// ```
pub struct FFRelayApi {
    client: Client,
    token: String,
}

const FFRELAY_API_ENDPOINT: &str = "https://relay.firefox.com/api";

const FFRELAY_EMAIL_ENDPOINT: &str = "v1/relayaddresses";
const FFRELAY_EMAIL_DOMAIN_ENDPOINT: &str = "v1/domainaddresses";

impl FFRelayApi {
    /// Creates a new Firefox Relay API client.
    ///
    /// # Arguments
    ///
    /// * `token` - Your Firefox Relay API token
    ///
    /// # Example
    ///
    /// ```
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// let api = FFRelayApi::new("your-api-token");
    /// ```
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let client = Client::new();

        Self {
            client,
            token: token.into(),
        }
    }

    /// Enables or disables an email relay via the specified API endpoint.
    ///
    /// This is a private helper function used by `enable()` and `disable()`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to use (either standard or domain relays)
    /// * `email_id` - The unique ID of the relay to toggle
    /// * `enabled` - Whether to enable (`true`) or disable (`false`) the relay
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or is rejected by the server.
    async fn toggle_with_endpoint(
        &self,
        endpoint: &str,
        email_id: u64,
        enabled: bool,
    ) -> Result<()> {
        let token = format!("Token {}", &self.token);
        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}/{email_id}/");

        info!("url: {url}");

        let request = FirefoxEmailRelayRequest::builder().enabled(enabled).build();

        let ret = self
            .client
            .patch(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .json(&request)
            .send()
            .await?;

        if ret.status().is_success() {
            Ok(())
        } else {
            Err(Error::EmailUpdateFailure {
                http_status: ret.status().as_u16(),
            })
        }
    }

    async fn create_with_endpoint(
        &self,
        endpoint: &str,
        request: FirefoxEmailRelayRequest,
    ) -> Result<String> {
        let token = format!("Token {}", &self.token);
        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}/");

        info!("url: {url}");

        let resp_dict = self
            .client
            .post(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .json(&request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&resp_dict);

        let res: FirefoxEmailRelay = serde_json::from_value(resp_dict)?;

        Ok(res.full_address)
    }

    async fn list_with_endpoint(&self, endpoint: &str) -> Result<Vec<FirefoxEmailRelay>> {
        let token = format!("Token {}", &self.token);

        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}");

        let relay_array = self
            .client
            .get(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&relay_array);

        let email_relays: Vec<FirefoxEmailRelay> = serde_json::from_value(relay_array)?;

        Ok(email_relays)
    }

    async fn delete_with_endpoint(&self, endpoint: &str, email_id: u64) -> Result<()> {
        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}/{email_id}");

        let token = format!("Token {}", &self.token);

        let ret = self
            .client
            .delete(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?;

        if ret.status().is_success() {
            Ok(())
        } else {
            Err(Error::EmailDeletionFailure {
                http_status: ret.status().as_u16(),
            })
        }
    }

    async fn find_email_relay(&self, email_id: u64) -> Result<FirefoxEmailRelay> {
        let relays = self.list().await?;

        for r in relays {
            if r.id == email_id {
                return Ok(r);
            }
        }

        Err(Error::RelayIdNotFound)
    }

    ////////////////////////////////////////////////////////////////////////////
    // PUBLIC
    ////////////////////////////////////////////////////////////////////////////

    /// Retrieves all Firefox Relay profiles associated with the API token.
    ///
    /// Returns detailed information about your Firefox Relay account including
    /// subscription status, usage statistics, and settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    /// let profiles = api.profiles().await?;
    /// for profile in profiles {
    ///     println!("Total masks: {}", profile.total_masks);
    ///     println!("Has premium: {}", profile.has_premium);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn profiles(&self) -> Result<Vec<FirefoxRelayProfile>> {
        let url = "https://relay.firefox.com/api/v1/profiles/";
        let token = format!("Token {}", &self.token);

        let profiles_dict = self
            .client
            .get(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&profiles_dict);

        let profiles: Vec<FirefoxRelayProfile> = serde_json::from_value(profiles_dict)?;

        Ok(profiles)
    }

    /// Creates a new email relay (alias).
    ///
    /// Creates either a random relay (ending in @mozmail.com) or a custom domain
    /// relay if you have a premium subscription and provide an address.
    ///
    /// # Arguments
    ///
    /// * `request` - Configuration for the new relay including description and optional custom address
    ///
    /// # Returns
    ///
    /// The full email address of the newly created relay.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP request fails, the response cannot be parsed,
    /// or you've reached your relay limit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    /// use ffrelay_api::types::FirefoxEmailRelayRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    ///
    /// // Create a random relay
    /// let request = FirefoxEmailRelayRequest::builder()
    ///     .description("For shopping sites".to_string())
    ///     .build();
    /// let email = api.create(request).await?;
    /// println!("Created: {}", email);
    ///
    /// // Create a custom domain relay (requires premium)
    /// let request = FirefoxEmailRelayRequest::builder()
    ///     .description("Newsletter".to_string())
    ///     .address("newsletter".to_string())
    ///     .build();
    /// let email = api.create(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: FirefoxEmailRelayRequest) -> Result<String> {
        let endpoint = if request.address.is_some() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.create_with_endpoint(endpoint, request).await
    }

    /// Lists all email relays (both random and domain relays).
    ///
    /// Retrieves all active email relays associated with your account,
    /// including both standard relays (@mozmail.com) and custom domain relays.
    ///
    /// # Returns
    ///
    /// A vector of all email relays with their statistics and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error only if both standard and domain relay requests fail.
    /// If one succeeds, returns the available relays.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    /// let relays = api.list().await?;
    /// for relay in relays {
    ///     println!("{}: {} (forwarded: {})",
    ///         relay.id,
    ///         relay.full_address,
    ///         relay.num_forwarded
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<FirefoxEmailRelay>> {
        let mut relays = vec![];

        if let Ok(email_relays) = self.list_with_endpoint(FFRELAY_EMAIL_ENDPOINT).await {
            relays.extend(email_relays);
        }

        if let Ok(domain_relays) = self.list_with_endpoint(FFRELAY_EMAIL_DOMAIN_ENDPOINT).await {
            relays.extend(domain_relays);
        }

        Ok(relays)
    }

    /// Deletes an email relay by its ID.
    ///
    /// Permanently removes the specified email relay. The relay will stop
    /// forwarding emails immediately. This action cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `email_id` - The unique ID of the relay to delete
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The relay ID is not found
    /// - The HTTP request fails
    /// - The deletion request is rejected by the server
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    ///
    /// // Delete a relay by ID
    /// api.delete(12345678).await?;
    /// println!("Relay deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, email_id: u64) -> Result<()> {
        let relay = self.find_email_relay(email_id).await?;

        let endpoint = if relay.is_domain() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.delete_with_endpoint(endpoint, email_id).await
    }

    /// Disables an email relay by its ID.
    ///
    /// When a relay is disabled, it will stop forwarding emails but remain in your
    /// account. You can re-enable it later without losing its statistics or configuration.
    ///
    /// # Arguments
    ///
    /// * `email_id` - The unique ID of the relay to disable
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The relay ID is not found
    /// - The HTTP request fails
    /// - The update request is rejected by the server
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    ///
    /// // Disable a relay temporarily
    /// api.disable(12345678).await?;
    /// println!("Relay disabled successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disable(&self, email_id: u64) -> Result<()> {
        let relay = self.find_email_relay(email_id).await?;

        let endpoint = if relay.is_domain() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.toggle_with_endpoint(endpoint, email_id, false).await
    }

    /// Enables an email relay by its ID.
    ///
    /// When a relay is enabled, it will start forwarding emails to your real email address.
    /// This is useful for re-enabling a previously disabled relay.
    ///
    /// # Arguments
    ///
    /// * `email_id` - The unique ID of the relay to enable
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The relay ID is not found
    /// - The HTTP request fails
    /// - The update request is rejected by the server
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ffrelay_api::api::FFRelayApi;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let api = FFRelayApi::new("your-api-token");
    ///
    /// // Enable a previously disabled relay
    /// api.enable(12345678).await?;
    /// println!("Relay enabled successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn enable(&self, email_id: u64) -> Result<()> {
        let relay = self.find_email_relay(email_id).await?;

        let endpoint = if relay.is_domain() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.toggle_with_endpoint(endpoint, email_id, true).await
    }
}
