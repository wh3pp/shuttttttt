use super::types::SortBy;
use crate::{error::Error, models::CommunityResponse};
use reqwest::Client;
use url::{Url, UrlQuery};

// --- Constants ---

/// The base URL for the Tunecore community API endpoint.
const CREATORS_API_BASE: &str = "https://www.tunecore.co.jp/api/v2/community";
/// The default page number to use for paginated requests.
const DEFAULT_PAGE: u32 = 1;
/// The default number of items to request per page.
const DEFAULT_PER_PAGE: u32 = 100;

// --- Builder ---

/// Builds and executes a request to the community songs endpoint.
///
/// This builder allows for a fluent, chainable interface to construct a complex
/// query for finding community songs based on various filters.
///
/// # Example
///
/// ```no_run
/// # use tunecore::TunecoreClient;
/// # use tunecore::creators::SortBy;
/// # async fn run() -> Result<(), tunecore::error::Error> {
/// let client = TunecoreClient::new();
///
/// let response = client
///     .creators()
///     .songs()
///     .page(2)
///     .per_page(50)
///     .artist_ids(&[123, 456])
///     .sort(SortBy::Popularity)
///     .send()
///     .await?;
///
/// # Ok(())
/// # }
/// ```
pub struct CreatorsBuilder<'a> {
    client: &'a Client,
    page: u32,
    per_page: u32,
    artist_ids: Vec<u64>,
    genre_ids: Vec<u16>,
    mood_ids: Vec<u16>,
    vocal: Option<bool>,
    instrumental: Option<bool>,
    duration_from: Option<u16>,
    duration_to: Option<u16>,
    share_rate_from: Option<u8>,
    share_rate_to: Option<u8>,
    bpm_from: Option<u16>,
    bpm_to: Option<u16>,
    sort: Option<SortBy>,
}

impl<'a> CreatorsBuilder<'a> {
    /// Creates a new `CreatorsBuilder` with default values.
    /// This is intended for internal use by the `CreatorsEndpoint`.
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            page: DEFAULT_PAGE,
            per_page: DEFAULT_PER_PAGE,
            artist_ids: Vec::new(),
            genre_ids: Vec::new(),
            mood_ids: Vec::new(),
            vocal: None,
            instrumental: None,
            duration_from: None,
            duration_to: None,
            share_rate_from: None,
            share_rate_to: None,
            bpm_from: None,
            bpm_to: None,
            sort: None,
        }
    }

    // --- Builder Methods ---

    /// Sets the page number for the request.
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Sets the number of results to return per page.
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page;
        self
    }

    /// Filters songs by one or more artist IDs.
    pub fn artist_ids(mut self, ids: &[u64]) -> Self {
        self.artist_ids.extend_from_slice(ids);
        self
    }

    /// Filters songs by one or more genre IDs.
    pub fn genre_ids(mut self, ids: &[u16]) -> Self {
        self.genre_ids.extend_from_slice(ids);
        self
    }

    /// Filters songs by one or more mood IDs.
    pub fn mood_ids(mut self, ids: &[u16]) -> Self {
        self.mood_ids.extend_from_slice(ids);
        self
    }

    /// Filters for songs that have vocals (`true`) or not (`false`).
    pub fn vocal(mut self, vocal: bool) -> Self {
        self.vocal = Some(vocal);
        self
    }

    /// Filters for songs that are instrumental (`true`) or not (`false`).
    pub fn instrumental(mut self, instrumental: bool) -> Self {
        self.instrumental = Some(instrumental);
        self
    }

    /// Sets the minimum duration for songs (in seconds).
    pub fn duration_from(mut self, duration: u16) -> Self {
        self.duration_from = Some(duration);
        self
    }

    /// Sets the maximum duration for songs (in seconds).
    pub fn duration_to(mut self, duration: u16) -> Self {
        self.duration_to = Some(duration);
        self
    }

    /// Sets the maximum revenue share rate for songs.
    pub fn share_rate_to(mut self, rate: u8) -> Self {
        self.share_rate_to = Some(rate);
        self
    }

    /// Sets the minimum revenue share rate for songs.
    pub fn share_rate_from(mut self, rate: u8) -> Self {
        self.share_rate_from = Some(rate);
        self
    }

    /// Sets the sorting order for the results.
    pub fn sort(mut self, order: SortBy) -> Self {
        self.sort = Some(order);
        self
    }

    /// Sets the minimum beats per minute (BPM) for songs.
    pub fn bpm_from(mut self, bpm: u16) -> Self {
        self.bpm_from = Some(bpm);
        self
    }

    /// Sets the maximum beats per minute (BPM) for songs.
    pub fn bpm_to(mut self, bpm: u16) -> Self {
        self.bpm_to = Some(bpm);
        self
    }

    /// Executes the request against the API.
    ///
    /// This consumes the builder and returns a `Result` containing either
    /// a `CommunityResponse` on success or an `Error` on failure.
    pub async fn send(self) -> Result<CommunityResponse, Error> {
        let url_path = format!("{CREATORS_API_BASE}/songs");
        let mut url = Url::parse(&url_path)?;

        self.build_url(&mut url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<CommunityResponse>()
            .await
            .map_err(Error::from)
    }

    // --- Private Helper Methods ---

    /// Assembles the final URL with all specified query parameters.
    fn build_url(&self, url: &mut Url) {
        let mut query = url.query_pairs_mut();

        query.append_pair("page", &self.page.to_string());
        query.append_pair("per_page", &self.per_page.to_string());

        self.append_vec_param(&mut query, "artist_ids", &self.artist_ids);
        self.append_vec_param(&mut query, "genre_ids", &self.genre_ids);
        self.append_vec_param(&mut query, "mood_ids", &self.mood_ids);

        self.append_optional_param(&mut query, "vocal", self.vocal);
        self.append_optional_param(&mut query, "instrumental", self.instrumental);
        self.append_optional_param(&mut query, "duration_from", self.duration_from);
        self.append_optional_param(&mut query, "duration_to", self.duration_to);
        self.append_optional_param(&mut query, "share_rate_from", self.share_rate_from);
        self.append_optional_param(&mut query, "share_rate_to", self.share_rate_to);
        self.append_optional_param(&mut query, "bpm_from", self.bpm_from);
        self.append_optional_param(&mut query, "bpm_to", self.bpm_to);

        if let Some(sort_param) = self.sort {
            let sort_str = match sort_param {
                SortBy::Popularity => "popular_rank:desc",
                SortBy::ShareRateDescending => "share_rate:desc",
                SortBy::ShareRateAscending => "share_rate:asc",
            };
            query.append_pair("sort", sort_str);
        }
    }

    /// A generic helper to append an optional parameter to the query string.
    /// If the value is `None`, nothing is appended.
    fn append_optional_param<T: ToString>(
        &self,
        query: &mut url::form_urlencoded::Serializer<'_, UrlQuery<'_>>,
        key: &str,
        value: Option<T>,
    ) {
        if let Some(val) = value {
            query.append_pair(key, &val.to_string());
        }
    }

    /// A generic helper to append a slice of values as repeated query parameters.
    fn append_vec_param<T: ToString>(
        &self,
        query: &mut url::form_urlencoded::Serializer<'_, UrlQuery<'_>>,
        key: &str,
        values: &[T],
    ) {
        for value in values {
            query.append_pair(key, &value.to_string());
        }
    }
}
