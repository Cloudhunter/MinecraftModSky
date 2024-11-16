extern crate minecraftmodsky;

use std::sync::Arc;

use anyhow::Result;
use env_logger::Env;
use log::info;

use minecraftmodsky::algos::{AlgosBuilder, MinecraftModSky};
use minecraftmodsky::config::Config;
use minecraftmodsky::processes::{FeedServer, PostIndexer, ProfileClassifier};
use minecraftmodsky::services::{Bluesky, Database};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Loading configuration");

    let config = Arc::new(Config::load()?);

    info!("Initializing service clients");

    let bluesky = Arc::new(Bluesky::unauthenticated());
    let database = Arc::new(Database::connect(&config.database_url).await?);

    let algos = Arc::new(
        AlgosBuilder::new()
            .add(
                "minecraftmodsky",
                MinecraftModSky::new(database.clone()),
            )
            .build(),
    );

    let post_indexer = PostIndexer::new(
        database.clone(),
        bluesky.clone(),
        algos.clone(),
        config.clone(),
    );
    let profile_classifier = ProfileClassifier::new(database.clone(), bluesky.clone());
    let feed_server = FeedServer::new(database.clone(), config.clone(), algos.clone());

    info!("Starting everything up");

    let _ = tokio::try_join!(
        tokio::spawn(post_indexer.start()),
        tokio::spawn(profile_classifier.start()),
        tokio::spawn(feed_server.serve()),
    )?;

    Ok(())
}
