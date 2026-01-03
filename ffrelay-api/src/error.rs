use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Http Error {http_status}")]
    RequestFailure { http_status: u16 },
    //
    // 3rd party
    //
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}
