//! The main database module, acting as a connection manager and repository factory.

mod collections;
mod error;
mod songs_repo;

#[allow(unused_imports)]
pub use error::{Error, Result};
pub use songs_repo::SongsRepo;

use mongodb::{Client, Database};

/// Represents the main database connection handler.
///
/// An instance of `Db` is used to establish a connection and gain
/// access to specialized repositories for each collection.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Db {
    database: Database,
}

impl Db {
    /// Establishes a connection to the MongoDB database.
    ///
    /// # Arguments
    /// * `uri` - The MongoDB connection string.
    /// * `db_name` - The name of the database to use.
    #[allow(dead_code)]
    pub async fn connect(uri: &str, db_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database(db_name);
        Ok(Self { database })
    }

    /// Returns a repository for interacting with the `songs` collection.
    #[allow(dead_code)]
    pub fn songs(&self) -> SongsRepo {
        SongsRepo::new(&self.database)
    }
}
