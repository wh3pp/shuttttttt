use super::song::CommunitySong;
use serde::{Deserialize, Serialize};

/// Represents the top-level response from the community songs endpoint.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CommunityResponse {
    /// A list of songs returned by the API query.
    pub community_songs: Vec<CommunitySong>,
    /// The total number of songs available that match the query.
    pub total: usize,
}
