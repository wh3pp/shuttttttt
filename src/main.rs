mod db;
mod ingestion;
use db::{Db, DbResult};
use dotenvy::dotenv;
use std::env;
use tunecore::{creators::SortBy, TunecoreClient};

#[tokio::main]
async fn main() -> DbResult<()> {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set in .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set in .env file");

    let db = Db::connect(&db_uri, &db_name).await?;
    let songs_repo = db.songs();

    let client = TunecoreClient::new();

    let response = client
        .creators()
        .songs()
        .per_page(20)
        .sort(SortBy::Popularity)
        .send()
        .await?;

    if response.community_songs.is_empty() {
        println!("No response were found.");
    } else {
        println!("Found {} songs", response.community_songs.len());
        songs_repo.save_many(&response.community_songs).await?;
    }
    Ok(())
}
