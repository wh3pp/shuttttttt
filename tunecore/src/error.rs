//! Defines the single, unified error type for the entire library.
//!
//! This module contains the `Error` enum, which represents all possible
//! errors that can occur during the library's operation. Using a single
//! error type allows for easier error handling by the user of the library.

use thiserror::Error;

/// Represents all possible errors that can occur in this library.
#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred during an HTTP request.
    /// This could be a network error, a timeout, or an HTTP status code
    /// indicating a client or server error.
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// An error occurred while parsing a URL.
    /// This is unlikely to happen with the hardcoded URLs but is included
    /// for completeness.
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// An error occurred while parsing a JSON response from the API.
    /// This typically happens if the API response format changes and no longer
    /// matches the expected data models.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// An unknown or unexpected error occurred.
    #[error("Unknown error")]
    Unknown,
}

