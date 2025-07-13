use tunecore::{creators::SortBy, Error, TunecoreClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = TunecoreClient::new();

    let response = client
        .creators()
        .songs()
        .per_page(10)
        .sort(SortBy::Popularity)
        .send()
        .await?;

    if response.community_songs.is_empty() {
        println!("No response were found.");
    } else {
        println!("Found {} songs", response.community_songs.len());
    }
    Ok(())
}
