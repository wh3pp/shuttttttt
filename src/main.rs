use mongodb::{Client, Collection};
use tunecore::{CommunitySong, CreatorsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_uri = "mongodb://localhost:27017";
    let mongo_client = Client::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("tunecore_db");
    let collection: Collection<CommunitySong> = db.collection("songs");

    println!("Connected to the database");

    let client = CreatorsClient::new();

    let response = client.scrape().per_page(5).send().await?;

    if response.community_songs.is_empty() {
        println!("No songs were found.");
    } else {
        for song in response.community_songs {
            println!("Title: {}", song.song_title.en);
            println!("Artist: {}", song.artist_name.en);
            println!("Link: {}", song.linkcore_url);

            collection.insert_one(&song).await?;
            println!("Guardado exitoso!");
            println!();
        }
    }

    Ok(())
}
