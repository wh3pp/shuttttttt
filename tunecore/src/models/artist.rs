use serde::{Deserialize, Serialize};

/// Represents the localized names for an artist.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ArtistName {
    /// The artist's name in Japanese.
    pub ja: String,
    /// The artist's name in English.
    pub en: String,
    /// The artist's name in Japanese (Kana).
    pub ja_kana: String,
}

/// Represents an artist associated with a song.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Artist {
    /// The unique identifier for the artist.
    pub artist_id: u64,
    /// The localized names of the artist.
    pub name: ArtistName,
    /// Indicates if this is a common artist entity.
    pub is_common_artist: bool,
    /// Indicates if a dedicated page for this artist is available.
    pub is_artist_page_available: bool,
    /// The URL path to the artist's page.
    pub artist_page_path: String,
    /// The ID of the common artist, if applicable.
    pub common_artist_id: Option<u64>,
}
