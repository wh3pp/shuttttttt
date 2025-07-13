//! Defines the custom error types for all database operations.

use thiserror::Error;

/// The enum for all possible database-related errors in the application.

#[derive(Error, Debug)]
pub enum DbError {
    /// Represents an error originating from the MongoDB driver.
    #[error("MongoDB error: {0}")]
    MongoDb(#[from] mongodb::error::Error),

    /// Representsesents an error during BSON serialization or deserialization.
    #[error("Bson error: {0}")]
    Bson(#[from] mongodb::bson::ser::Error),

    /// Represents an error originating from the Tunecore client library.
    #[error("Tunecore error: {0}")]
    Tunecore(#[from] tunecore::error::Error),

    /// Represents an error originating from the env error
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
}
