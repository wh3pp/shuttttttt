use tunecore::{CreatorsClient, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = CreatorsClient::new();

    let response = client.scrape().per_page(5).send().await?;
    //let response = client.scrape().send().await?;
    //let response = client.scrape().page(100).per_page(1).send().await?;

    if response.community_songs.is_empty() {
        println!("No songs were found.");
    } else {
        for song in response.community_songs {
            println!("Title: {}", song.song_title.en);
            println!("Artist: {}", song.artist_name.en);
            println!("Link: {}", song.linkcore_url);
            println!();
        }
    }

    Ok(())
}
