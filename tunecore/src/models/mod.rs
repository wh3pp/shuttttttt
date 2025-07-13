//! Contains the data models used to represent entities and API responses from the Tunecore API.
//!
//! This module organizes all data structures into logical sub-modules:
//! - [`artist`]: Models related to artists.
//! - [`song`]: Models related to songs.
//! - [`response`]: Models that represent top-level API responses.
//!
//! The most common models are re-exported at the crate's root for convenient access.

// Declare the sub-modules for organization.
pub mod artist;
pub mod response;
pub mod song;

// Re-export the primary models to the top level of the `models` module.
pub use artist::{Artist, ArtistName};
pub use response::CommunityResponse;
pub use song::{CommunitySong, SongTitle};
