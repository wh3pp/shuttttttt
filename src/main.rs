mod commands;
mod db;
mod error;
mod ingestion;

use commands::commands;
use dotenvy::dotenv;
use error::Result;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tunecore::TunecoreClient;

pub struct Data {
    pub tunecore_client: tunecore::TunecoreClient,
}
type Context<'a> = poise::Context<'a, Data, error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default tracing subscriber failed");

    dotenv().ok();

    info!("Loading configuration...");

    let token = env::var("DISCORD_TOKEN")?;
    let guild_id: u64 = env::var("GUILD_ID")?
        .parse::<u64>()
        .expect("Invalid guild id");

    let prefix = "!";

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(prefix.into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId::new(guild_id),
                )
                .await?;
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    tunecore_client: TunecoreClient::new(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
    Ok(())
}
