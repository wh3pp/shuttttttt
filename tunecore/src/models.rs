use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SongTitle {
    pub ja: String,
    pub en: String,
    pub ja_kana: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ArtistName {
    pub ja: String,
    pub en: String,
    pub ja_kana: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Artist {
    pub artist_id: u64,
    pub name: ArtistName,
    pub is_common_artist: bool,
    pub is_artist_page_available: bool,
    pub artist_page_path: String,
    pub common_artist_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CommunitySong {
    pub id: u64,
    pub index: usize,
    pub audio_url: Option<String>,
    pub youtube_art_track_url: Option<String>,
    pub linkcore_url: String,
    pub bpm: f32,
    pub duration: f32,
    pub genre_id: Vec<u16>,
    pub mood_id: u16,
    pub jacket_url: String,
    pub street_date: NaiveDate,
    pub song_title: SongTitle,
    pub artist_name: ArtistName,
    pub artists: Vec<Artist>,
    pub channel_share_percent_str: String,
    pub is_favorite: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Creators {
    pub community_songs: Vec<CommunitySong>,
    pub total: usize,
}
