//! Tagesschau — German news search via tagesschau.de JSON API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde::Deserialize;

pub struct Tagesschau {
    metadata: EngineMetadata,
    client: Client,
}

impl Tagesschau {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "tagesschau".to_string(),
                display_name: "Tagesschau".to_string(),
                homepage: "https://www.tagesschau.de".to_string(),
                categories: vec![SearchCategory::News],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    search_results: Option<Vec<SearchItem>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchItem {
    title: Option<String>,
    details_web: Option<String>,
    first_sentence: Option<String>,
    date: Option<String>,
}

#[async_trait]
impl SearchEngine for Tagesschau {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page_size = 10;
        let page_index = query.page.saturating_sub(1);
        let url = format!(
            "https://www.tagesschau.de/api2u/search/?searchText={}&pageSize={}&pageIndex={}&resultPage=all",
            urlencoding::encode(&query.query),
            page_size,
            page_index
        );

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Tagesschau request failed: {e}")))?;

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Tagesschau parse failed: {e}")))?;

        let results = api
            .search_results
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let title = item.title.filter(|t| !t.is_empty())?;
                let result_url = item.details_web.filter(|u| !u.is_empty())?;
                let mut snippet = item.first_sentence.unwrap_or_default();
                if let Some(date) = &item.date {
                    if !date.is_empty() {
                        snippet = format!("{} — {}", date, snippet);
                    }
                }
                Some(SearchResult {
                    title,
                    url: result_url,
                    snippet,
                    engine: "tagesschau".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail_url: None,
                })
            })
            .collect();

        Ok(results)
    }
}
