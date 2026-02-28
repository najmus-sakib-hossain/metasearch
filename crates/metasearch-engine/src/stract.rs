//! Stract — independent web search engine via JSON POST API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct Stract {
    metadata: EngineMetadata,
    client: Client,
}

impl Stract {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "stract".to_string(),
                display_name: "Stract".to_string(),
                homepage: "https://stract.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Serialize)]
struct SearchRequest {
    query: String,
    page: u32,
    num_results: u32,
}

#[derive(Deserialize)]
struct ApiResponse {
    webpages: Option<Vec<WebPage>>,
}

#[derive(Deserialize)]
struct WebPage {
    title: Option<String>,
    url: Option<String>,
    snippet: Option<String>,
}

#[async_trait]
impl SearchEngine for Stract {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let body = SearchRequest {
            query: query.query.clone(),
            page: query.page.saturating_sub(1),
            num_results: 10,
        };

        let resp = self
            .client
            .post("https://stract.com/beta/api/search")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Stract request failed: {e}")))?;

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Stract parse failed: {e}")))?;

        let results = api
            .webpages
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .filter_map(|(i, page)| {
                let title = page.title.filter(|t| !t.is_empty())?;
                let url = page.url.filter(|u| !u.is_empty())?;
                Some(SearchResult {
                    title,
                    url,
                    snippet: page.snippet.unwrap_or_default(),
                    engine: "stract".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail_url: None,
                })
            })
            .collect();

        Ok(results)
    }
}
