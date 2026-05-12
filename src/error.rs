use thiserror::Error;

/// Errors returned by the AEO SDK.
#[derive(Debug, Error)]
pub enum AeoError {
    /// The document was not valid JSON or did not match the AEO schema.
    #[error("AEO document parse error: {0}")]
    Parse(#[from] serde_json::Error),

    /// HTTP transport error (only with `client` feature).
    #[cfg(feature = "client")]
    #[error("HTTP error: {0}")]
    Http(#[from] Box<ureq::Error>),

    /// I/O error reading a response body.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A 4xx or 5xx response was returned by the origin.
    #[error("AEO fetch failed: HTTP {status} from {url}")]
    HttpStatus {
        /// HTTP status code.
        status: u16,
        /// The well-known URL that returned the status.
        url: String,
    },
}
