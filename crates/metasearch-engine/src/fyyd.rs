//! Fyyd engine — search podcasts via Fyyd JSON API.
//! Translated from SearXNG `searx/engines/fyyd.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Fyyd {
    client: Client,
}

impl Fyyd {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Fyyd {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "fyyd".to_string(),
            display_name: "Fyyd".to_string(),
            categories: vec![SearchCategory::Music],
            enabled: true,
            weight: 0.6,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let page_index = (page as i32) - 1;
        let count = 10;

        let url = format!(
            "https://api.fyyd.de/0.2/search/podcast?term={}&count={}&page={}",
            urlencoding::encode(&query.query),
            count,
            page_index,
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
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
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::Music);
                result.thumbnail = item["smallImageURL"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
