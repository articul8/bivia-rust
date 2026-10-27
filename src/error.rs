/// Errors returned by the Bivia SDK.
#[derive(Debug, thiserror::Error)]
pub enum BiviaError {
    /// The API returned a non-success status code.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        status: u16,
        message: String,
        body: String,
    },

    /// A network or connection error from reqwest.
    #[error("connection error: {0}")]
    Connection(#[from] reqwest::Error),

    /// Failed to serialize a request or deserialize a response.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The `A8_API_KEY` environment variable is not set.
    #[error("A8_API_KEY environment variable is not set")]
    MissingApiKey,
}

/// Convenience alias used throughout the SDK.
pub type Result<T> = std::result::Result<T, BiviaError>;
