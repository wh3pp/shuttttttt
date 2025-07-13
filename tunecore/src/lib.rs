//! # A Rust client for the Tunecore API.
//!
//! This library provides a simple and fluent interface for interacting
//! with the Tunecore Japan community API endpoints.
//!
//! ## Example
//!
//! ```no_run
//! # use tunecore::{creators::SortBy, Error, TunecoreClient};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Error> {
//! // Create a new client with default settings.
//! let client = TunecoreClient::new();
//!
//! // Build and send a request to the community songs endpoint.
//! let response = client
//!     .creators()
//!     .songs()
//!     .per_page(10)
//!     .sort(SortBy::Popularity)
//!     .send()
//!     .await?;
//!
//! if response.community_songs.is_empty(){
//!     println!("No songs were found.");
//! } else{
//!     println!("Found {} songs.", response.community_songs.len());
//! }
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod models;

mod endpoints;

pub use endpoints::creators;
pub use error::Error;

use creators::CreatorsEndpoint;
use reqwest::Client;
/// The main entry point for interacting with the Tunecore API.
///
/// This client holds the HTTP client and provides access to different
/// API endpoint groups (e.g., `creators`).
#[derive(Debug, Clone)]
pub struct TunecoreClient {
    http_client: Client,
}

impl TunecoreClient {
    /// Creates a new `TunecoreClient` with a default `reqwest::Client`.
    ///
    /// This is suitable for most basic use cases.
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    /// Creates a new `TunecoreClient` with a user-provided `reqwest::Client`.
    ///
    /// This allows for custom configurations, such as setting timeouts,
    /// a proxy, or default headers.
    pub fn with_client(client: Client) -> Self {
        Self {
            http_client: client,
        }
    }

    /// Returns a handler for the "creators" API endpoints.
    pub fn creators(&self) -> CreatorsEndpoint<'_> {
        CreatorsEndpoint::new(&self.http_client)
    }
}

impl Default for TunecoreClient {
    /// Creates a default `TunecoreClient`.
    ///
    /// This is equivalent to calling `TunecoreClient::new()`.
    fn default() -> Self {
        Self::new()
    }
}
