use futures_util::stream::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use tunecore::CommunitySong;

type DbResult<T> = Result<T, mongodb::error::Error>;

#[derive(Clone, Debug)]
pub struct Db {
    collection: Collection<CommunitySong>,
}

impl Db {
    pub async fn connect(uri: &str) -> DbResult<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database("tunecore_db");
        let collection = database.collection("songs");
        println!("Connected to the database");
        Ok(Self { collection })
    }

    pub async fn save_songs(&self, songs: &[CommunitySong]) -> DbResult<()> {
        if songs.is_empty() {
            return Ok(());
        }

        self.collection.insert_many(songs).await?;
        Ok(())
    }

    pub async fn get_all_songs(&self) -> DbResult<Vec<CommunitySong>> {
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut songs = Vec::new();
        while let Some(song) = cursor.try_next().await? {
            songs.push(song);
        }
        Ok(songs)
    }

    pub async fn delete_all_songs(&self) -> DbResult<u64> {
        let result = self.collection.delete_many(doc! {}).await?;
        Ok(result.deleted_count)
    }
}
