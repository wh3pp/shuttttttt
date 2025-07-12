mod error;
mod models;

pub use error::Error;
pub use models::{Artist, CommunitySong, Creators, SongTitle};

use url::Url;

const API_BASE_URL: &str = "https://www.tunecore.co.jp/api/v2";

#[derive(Debug)]
pub struct CreatorsClient {
    http_client: reqwest::Client,
}

impl CreatorsClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    pub fn scrape(&self) -> CreatorsScrapeBuilder<'_> {
        CreatorsScrapeBuilder::new(self)
    }
}

pub struct CreatorsScrapeBuilder<'a> {
    client: &'a CreatorsClient,
    page: u32,
    per_page: u32,
}

impl<'a> CreatorsScrapeBuilder<'a> {
    fn new(client: &'a CreatorsClient) -> Self {
        Self {
            client,
            page: 1,
            per_page: 100,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page;
        self
    }

    pub async fn send(self) -> Result<Creators, Error> {
        let url_path = format!("{}/community/songs", &API_BASE_URL);
        let mut url = Url::parse(&url_path)?;

        url.query_pairs_mut()
            .append_pair("page", &self.page.to_string())
            .append_pair("per_page", &self.per_page.to_string());

        let creators_response = self
            .client
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Creators>()
            .await?;

        Ok(creators_response)
    }
}

impl Default for CreatorsClient {
    fn default() -> Self {
        Self::new()
    }
}
