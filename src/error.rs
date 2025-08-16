use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Db(#[from] crate::db::Error),

    #[error("Serenity error: {0}")]
    Serenity(Box<poise::serenity_prelude::Error>),

    #[error("Tunecore error: {0}")]
    Tunecore(#[from] tunecore::Error),

    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
}

impl From<poise::serenity_prelude::Error> for BotError {
    fn from(error: poise::serenity_prelude::Error) -> Self {
        BotError::Serenity(Box::new(error))
    }
}

pub type Error = BotError;
pub type Result<T> = std::result::Result<T, Error>;



