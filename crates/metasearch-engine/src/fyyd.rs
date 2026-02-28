//! Fyyd engine — search podcasts via Fyyd JSON API.
//! Translated from SearXNG `searx/engines/fyyd.py`.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct Fyyd {
    metadata: EngineMetadata,
    client: Client,
}

impl Fyyd {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "fyyd".to_string(),
                display_name: "Fyyd".to_string(),
                homepage: "https://fyyd.de".to_string(),
                categories: vec![SearchCategory::Music],
                enabled: true,
                timeout_ms: 5000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Fyyd {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let page_index = (page as i32) - 1;
        let count = 10;

        let url = format!(
            "https://api.fyyd.de/0.2/search/podcast?term={}&count={}&page={}",
            urlencoding::encode(&query.query),
            count,
            page_index,
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(items) = data["data"].as_array() {
            for (i, item) in items.iter().enumerate() {
                let title = item["title"].as_str().unwrap_or_default();
                let link = item["htmlURL"].as_str().unwrap_or_default();
                let description = item["description"].as_str().unwrap_or_default();
                let episode_count = item["episode_count"].as_u64().unwrap_or(0);

                let snippet = if description.is_empty() {
                    format!("{} episodes", episode_count)
                } else {
                    let desc_short = if description.len() > 200 {
                        format!("{}...", &description[..200])
                    } else {
                        description.to_string()
                    };
                    format!("{} | {} episodes", desc_short, episode_count)
                };

                let mut result = SearchResult::new(
                    title.to_string(),
                    link.to_string(),
                    snippet,
                    "fyyd".to_string(),
                );
                result.engine_rank = (i + 1) as u32;
                result.category = SearchCategory::Music.to_string();
                result.thumbnail = item["smallImageURL"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
