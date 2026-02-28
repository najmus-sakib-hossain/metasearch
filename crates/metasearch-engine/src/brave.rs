//! Brave Search engine implementation.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::Result,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use tracing::warn;

#[allow(dead_code)]
pub struct Brave {
    metadata: EngineMetadata,
    client: Client,
    api_key: Option<String>,
}

impl Brave {
    pub fn new(client: Client, api_key: Option<String>) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "brave".to_string(),
                display_name: "Brave Search".to_string(),
                homepage: "https://search.brave.com".to_string(),
                categories: vec![SearchCategory::General, SearchCategory::News],
                enabled: true,
                timeout_ms: 3000,
                weight: 1.3,
            },
            client,
            api_key,
        }
    }
}

#[async_trait]
impl SearchEngine for Brave {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // TODO: Implement Brave Search API (JSON API available with key)
        warn!(engine = "brave", query = %query.query, "Engine not yet implemented");
        Ok(Vec::new())
    }
}
