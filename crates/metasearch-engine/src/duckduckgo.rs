//! DuckDuckGo search engine implementation.

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

pub struct DuckDuckGo {
    metadata: EngineMetadata,
    client: Client,
}

impl DuckDuckGo {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "duckduckgo".to_string(),
                display_name: "DuckDuckGo".to_string(),
                homepage: "https://duckduckgo.com".to_string(),
                categories: vec![
                    SearchCategory::General,
                    SearchCategory::Images,
                    SearchCategory::News,
                ],
                enabled: true,
                timeout_ms: 4000,
                weight: 1.2,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for DuckDuckGo {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // TODO: Implement DuckDuckGo HTML parsing
        // DuckDuckGo offers a lite HTML version ideal for scraping.
        warn!(engine = "duckduckgo", query = %query.query, "Engine not yet implemented");
        Ok(Vec::new())
    }
}
