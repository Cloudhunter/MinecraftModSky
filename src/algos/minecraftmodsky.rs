use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::Algo;

use crate::services::bluesky;
use crate::services::database::{self, Database};

/// An algorithm that serves posts written in Russian by people living in Netherlands
pub struct MinecraftModSky {
    database: Arc<Database>,
}

impl MinecraftModSky {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database,
        }
    }
}

impl MinecraftModSky {
    async fn is_profile_minecraft_modder(&self, did: &str) -> Result<bool> {
        Ok(self.database.is_profile_minecraft_modder(did).await? == Some(true))
    }
}

#[async_trait]
impl Algo for MinecraftModSky {
    async fn should_index_post(
        &self,
        author_did: &str,
        _post: &bluesky::PostRecord,
    ) -> Result<bool> {
        Ok(self.is_profile_minecraft_modder(author_did).await?)
    }

    async fn fetch_posts(
        &self,
        database: &Database,
        limit: u8,
        earlier_than: Option<(DateTime<Utc>, &str)>,
    ) -> Result<Vec<database::Post>> {
        Ok(database
            .fetch_posts_by_authors_country("nl", limit as usize, earlier_than)
            .await?)
    }
}
