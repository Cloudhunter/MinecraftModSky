use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use log::{error, info};

use crate::services::Bluesky;
use crate::services::Database;

use fancy_regex::Regex;

pub struct ProfileClassifier {
    database: Arc<Database>,
    bluesky: Arc<Bluesky>,
}

impl ProfileClassifier {
    pub fn new(database: Arc<Database>, bluesky: Arc<Bluesky>) -> Self {
        Self {
            database,
            bluesky,
        }
    }

    pub async fn start(self) -> Result<()> {
        info!("Starting");

        loop {
            if let Err(e) = self.classify_unclassified_profiles().await {
                error!("Problem with classifying profiles: {}", e)
            }

            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    async fn is_modder(
        &self,
        description: &str,
    ) -> Result<bool> {
        let re = Regex::new(r"^((?=.*\b.+scraft\b)(?=.*\b(?:mod.*?)\b)|(?=.*modrinth|curseforge)).*$").unwrap();
        let result = re.is_match(description);
        assert!(result.is_ok());
        Ok(result.unwrap())
    }
    async fn classify_unclassified_profiles(&self) -> Result<()> {
        // TODO: Maybe streamify this so that each thing is processed in parallel

        let dids = self.database.fetch_unprocessed_profile_dids().await?;

        if dids.is_empty() {
            info!("No profiles to process");
        } else {
            info!("Classifying {} new profiles", dids.len());
            for did in &dids {
                match self.fill_in_profile_details(did).await {
                    Ok(()) => continue,
                    Err(e) => error!("Could not classify profile with did {}: {:?}", did, e),
                }
            }
        }

        Ok(())
    }

    async fn fill_in_profile_details(&self, did: &str) -> Result<()> {
        let details = self
            .bluesky
            .fetch_profile_details(did)
            .await
            .context("Could not fetch profile details")?;

        let modder = match details {
            Some(details) => self
                .is_modder(&details.description)
                .await
                .context("Could not decide if is modder")?,
            None => false.to_owned()
        };

        self.database.store_profile_details(did, &modder).await?;
        info!("Stored modder status for {did}: {modder}");
        Ok(())
    }
}
