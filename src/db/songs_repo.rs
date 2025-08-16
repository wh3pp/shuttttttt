//! Contains the repository logic for interacting with the `songs` collection.
//! This module provides a structured and safe API for all database
//! operations related to `CommunitySong` documents.

use super::{collections, error::Result};
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, to_document},
    options::{FindOptions, UpdateModifications, UpdateOneModel, WriteModel},
    Collection, Database,
};
use tunecore::models::CommunitySong;

/// A repository for handling database operations on the `songs` collection.
///
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SongsRepo {
    database: Database,
    collection: Collection<CommunitySong>,
}

impl SongsRepo {
    /// Creates a new `SongsRepo`.
    ///
    /// # Arguments
    /// * `db` - A reference to the `mongodb::Database` instance.
    pub(super) fn new(db: &Database) -> Self {
        Self {
            database: db.clone(),
            collection: db.collection(collections::SONGS),
        }
    }

    /// Saves a slice of `CommunitySong` documents to the database using an "upsert" strategy.
    ///
    // If a song with the same `id` already exists, it will be updated.
    /// Otherwise, a new song document will be inserted. This prevents duplicates.
    /// For this to be efficient, create a unique index in MongoDB on the `id` field.
    ///
    /// # Arguments
    /// * `songs` - A slice of `CommunitySong` to save or update.
    pub async fn save_many(&self, songs: &[CommunitySong]) -> Result<()> {
        if songs.is_empty() {
            return Ok(());
        }

        let upserts = songs
            .iter()
            .map(|song| {
                let filter = doc! { "id": song.id as i64 };

                let update_doc = to_document(song)?;
                let update = UpdateModifications::Document(doc! { "$set": update_doc });

                let model = UpdateOneModel::builder()
                    .namespace(self.collection.namespace())
                    .filter(filter)
                    .update(update)
                    .upsert(true)
                    .build();

                Ok(WriteModel::UpdateOne(model))
            })
            .collect::<Result<Vec<_>>>()?;

        let client = self.database.client();

        client.bulk_write(upserts).await?;

        Ok(())
    }

    /// Retrieves a paginated list of songs from the collection.
    ///
    /// This method is the recommended way to fetch multiple documents, as it
    /// prevents high memory consumption by fetching a limited subset of data ("a page").
    ///
    /// # Arguments
    /// * `page` - The page number to retrieve (1-based). If 0 is passed, it defaults to 1.
    /// * `per_page` - The maximum number of songs to retrieve for the page.
    #[allow(dead_code)]
    pub async fn get_paged(&self, page: u64, per_page: u64) -> Result<Vec<CommunitySong>> {
        let page = page.max(1);
        // Calculate the number of documents to skip to get to the desired page.
        let skip = (page - 1) * per_page;

        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(per_page as i64)
            .build();

        let songs = self
            .collection
            .find(doc! {})
            .with_options(find_options)
            .await?
            .try_collect()
            .await?;

        Ok(songs)
    }

    /// Deletes all documents from the `songs` collection.
    ///
    /// ## Warning
    /// This is a destructive operation and will permanently remove all data
    /// from the collection.
    ///
    /// # Returns
    /// A `DbResult` containing the number of documents deleted.
    #[allow(dead_code)]
    pub async fn delete_all(&self) -> Result<u64> {
        let result = self.collection.delete_many(doc! {}).await?;
        Ok(result.deleted_count)
    }
}
