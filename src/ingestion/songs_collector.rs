use crate::db::{Result, SongsRepo};
use futures::stream;
use futures_util::{stream::TryStreamExt, StreamExt};
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};
use tunecore::{models::CommunitySong, TunecoreClient};

/// A collector to store all community songs from TuneCore.
///
/// This utility encapsulates the logic for fetching paginated data concurrently
/// and saving it to a data repository in efficient batches.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SongsCollector {
    client: TunecoreClient,
    songs_repo: SongsRepo,
}

impl SongsCollector {
    /// Creates a new instance of the song collector.
    #[allow(dead_code)]
    pub fn new(client: &TunecoreClient, songs_repo: &SongsRepo) -> Self {
        Self {
            client: client.clone(),
            songs_repo: songs_repo.clone(),
        }
    }

    /// Fetches all songs from the API concurrently and saves them to the database.
    #[instrument(skip_all, fields(concurrency = max_concurrency))]
    #[allow(dead_code)]
    pub async fn collect_all(&self, max_concurrency: usize) -> Result<()> {
        const BATCH_SIZE: usize = 1000;

        let first_response = self.client.creators().songs().send().await?;
        let total_songs = first_response.total;
        let per_page = first_response.community_songs.len();

        if per_page == 0 {
            warn!("API returned 0 songs on the first page. No data to collect.");
            return Ok(());
        }

        let total_pages = total_songs.div_ceil(per_page);
        info!(
            total_songs,
            per_page, total_pages, "Created collection plan."
        );

        self.songs_repo
            .save_many(&first_response.community_songs)
            .await?;
        debug!(page = 1, songs_saved = per_page, "Saved initial page.");

        if total_pages <= 1 {
            info!("Only one page of data found. Collection complete.");
            return Ok(());
        }

        let collector = Arc::new(self.clone());

        let final_batch = stream::iter(2..=total_pages as u32)
            .map(|page| {
                let collector_clone = Arc::clone(&collector);
                async move {
                    debug!(page, "Fetching page.");
                    let songs = collector_clone
                        .client
                        .creators()
                        .songs()
                        .page(page)
                        .send()
                        .await?
                        .community_songs;
                    Ok(songs) as Result<Vec<CommunitySong>>
                }
            })
            .buffer_unordered(max_concurrency)
            .try_fold(
                Vec::with_capacity(BATCH_SIZE),
                |mut batch, songs_page| async {
                    batch.extend(songs_page);
                    if batch.len() >= BATCH_SIZE {
                        info!(songs_in_batch = batch.len(), "Saving batch to database.");
                        collector.songs_repo.save_many(&batch).await?;
                        batch.clear();
                    }
                    Ok(batch)
                },
            )
            .await?;

        if !final_batch.is_empty() {
            info!(
                songs_in_batch = final_batch.len(),
                "Saving final batch to database."
            );
            self.songs_repo.save_many(&final_batch).await?;
        }

        Ok(())
    }
}
