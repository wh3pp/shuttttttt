use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("URL parsing error")]
    UrlParse(#[from] url::ParseError),

    #[error("Unknown error")]
    Unknown,
}
