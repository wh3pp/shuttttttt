//! Handles all API endpoints related to creators and their songs.
//!
//! This module provides the `CreatorsEndpoint` and the `CreatorsBuilder`
//! for constructing and sending requests.

mod builder;
mod types;

pub use builder::CreatorsBuilder;
pub use types::SortBy;

use reqwest::Client;

/// A handler for endpoints related to creators.
pub struct CreatorsEndpoint<'a> {
    http_client: &'a Client,
}

impl<'a> CreatorsEndpoint<'a> {
    /// Creates a new instance of the endpoint handler. (Internal use only)
    pub(crate) fn new(http_client: &'a Client) -> Self {
        Self { http_client }
    }

    /// Builds a request to fetch community songs.
    ///
    /// Returns a `CreatorsBuilder` to set filters and execute the request.
    pub fn songs(&self) -> CreatorsBuilder<'_> {
        CreatorsBuilder::new(self.http_client)
    }
}
