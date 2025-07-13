mod db;
mod ingestion;

use crate::db::{Db, DbResult};
use crate::ingestion::SongsCollector;
use dotenvy::dotenv;
use std::env;
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tunecore::TunecoreClient;

#[tokio::main]
async fn main() -> DbResult<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default tracing subscriber failed");

    dotenv().ok();

    info!("Loading configuration...");

    let db_uri = env::var("DATABASE_URI")?;
    let db_name = env::var("DATABASE_NAME")?;

    info!("Establishing connections...");
    let db = Db::connect(&db_uri, &db_name).await?;
    let songs_repo = db.songs();
    let client = TunecoreClient::new();
    let collector = SongsCollector::new(&client, &songs_repo);
    info!("Setup complete.");

    const MAX_CONCURRENCY: usize = 25;
    info!(concurrency = MAX_CONCURRENCY, "Starting song collection...");

    let start_time = Instant::now();
    collector.collect_all(MAX_CONCURRENCY).await?;
    let duration = start_time.elapsed();

    info!(
        duration_secs = duration.as_secs_f64(),
        "Collection finished successfully."
    );

    Ok(())
}
