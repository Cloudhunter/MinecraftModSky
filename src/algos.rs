mod minecraftmodsky;

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::services::bluesky;
use crate::services::database::{self, Database};

pub use self::minecraftmodsky::MinecraftModSky;

#[async_trait]
pub trait Algo {
    async fn should_index_post(&self, author_did: &str, post: &bluesky::PostRecord)
        -> Result<bool>;

    async fn fetch_posts(
        &self,
        database: &Database,
        limit: u8,
        earlier_than: Option<(DateTime<Utc>, &str)>,
    ) -> Result<Vec<database::Post>>;
}

pub type AnyAlgo = Box<dyn Algo + Sync + Send>;
type AlgosMap = HashMap<String, AnyAlgo>;

pub struct Algos {
    algos: AlgosMap,
}

impl Algos {
    pub fn iter_names(&self) -> impl Iterator<Item = &str> {
        self.algos.keys().map(String::as_str)
    }

    pub fn iter_all(&self) -> impl Iterator<Item = &AnyAlgo> {
        self.algos.values()
    }

    pub fn get_by_name(&self, name: &str) -> Option<&AnyAlgo> {
        self.algos.get(name)
    }
}

#[derive(Default)]
pub struct AlgosBuilder {
    algos: AlgosMap,
}

impl AlgosBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add<T: Algo + Send + Sync + 'static>(mut self, name: &str, algo: T) -> Self {
        self.algos.insert(name.to_owned(), Box::new(algo));
        self
    }

    pub fn build(self) -> Algos {
        Algos { algos: self.algos }
    }
}
