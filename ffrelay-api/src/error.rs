//! Error types for the Firefox Relay API client.

use thiserror::Error;

/// A specialized `Result` type for Firefox Relay API operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur when interacting with the Firefox Relay API.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed with the given status code.
    ///
    /// This typically indicates a server error or invalid request.
    #[error("Http Error {http_status}")]
    RequestFailure { http_status: u16 },

    /// The specified relay ID was not found in your account.
    ///
    /// This occurs when trying to delete or access a relay that doesn't exist
    /// or doesn't belong to your account.
    #[error("Email Id not found")]
    RelayIdNotFound,

    /// Failed to delete the email relay.
    ///
    /// The server rejected the deletion request. Check the status code for details.
    #[error("Deletion Failure. Status code: {http_status}")]
    EmailDeletionFailure { http_status: u16 },

    //
    // 3rd party errors
    //
    /// An HTTP client error occurred.
    ///
    /// This wraps errors from the `reqwest` HTTP client library,
    /// such as network connectivity issues or timeout errors.
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),

    /// Failed to serialize or deserialize JSON data.
    ///
    /// This typically indicates an unexpected API response format.
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}
