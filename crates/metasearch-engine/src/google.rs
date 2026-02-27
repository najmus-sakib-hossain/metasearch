//! Google search engine implementation.

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

pub struct Google {
    metadata: EngineMetadata,
    client: Client,
}

impl Google {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "google".to_string(),
                display_name: "Google".to_string(),
                homepage: "https://www.google.com".to_string(),
                categories: vec![SearchCategory::General, SearchCategory::Images, SearchCategory::News],
                enabled: true,
                timeout_ms: 4000,
                weight: 1.5,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Google {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // TODO: Implement Google scraping / API integration
        // This is a stub — real implementation would parse Google's HTML
        // or use the Custom Search JSON API.
        warn!(engine = "google", query = %query.query, "Engine not yet implemented, returning empty results");
        Ok(Vec::new())
    }
}
