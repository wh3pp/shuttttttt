use super::artist::{Artist, ArtistName};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents the localized titles for a song.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SongTitle {
    /// The song title in Japanese.
    pub ja: String,
    /// The song title in English.
    pub en: Option<String>,
    /// The song title in Japanese (Kana).
    pub ja_kana: Option<String>,
}

/// Represents a single song from the community endpoint.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CommunitySong {
    /// The unique identifier for the song.
    pub id: u64,
    /// The index of the song in the current response list.
    pub index: usize,
    /// The URL to the audio file for previewing the song.
    pub audio_url: Option<String>,
    /// The URL to the YouTube art track for the song.
    pub youtube_art_track_url: Option<String>,
    /// A Linkfire URL for the song.
    pub linkcore_url: String,
    /// The beats per minute (BPM) of the song.
    pub bpm: f32,
    /// The duration of the song in seconds.
    pub duration: f32,
    /// A list of genre IDs associated with the song.
    pub genre_id: Vec<u16>,
    /// The mood ID associated with the song.
    pub mood_id: u16,
    /// The URL to the album/song cover art.
    pub jacket_url: String,
    /// The release date of the song.
    pub street_date: NaiveDate,
    /// The localized titles of the song.
    pub song_title: SongTitle,
    /// The localized names of the primary artist.
    pub artist_name: ArtistName,
    /// A list of all artists credited on the song.
    pub artists: Vec<Artist>,
    /// The revenue share percentage for the channel, as a string.
    pub channel_share_percent_str: String,
    /// Indicates if the current user has favorited this song.
    pub is_favorite: bool,
}
