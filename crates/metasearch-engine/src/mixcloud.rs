//! Mixcloud engine — search music mixes via Mixcloud JSON API.
//! Translated from SearXNG `searx/engines/mixcloud.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Mixcloud {
    client: Client,
}

impl Mixcloud {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Mixcloud {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "mixcloud".to_string(),
            display_name: "Mixcloud".to_string(),
            categories: vec![SearchCategory::Music],
            enabled: true,
            weight: 0.7,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let offset = (page as u32 - 1) * 10;

        let url = format!(
            "https://api.mixcloud.com/search/?q={}&type=cloudcast&limit=10&offset={}",
            urlencoding::encode(&query.query),
            offset,
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
                let title = item["name"].as_str().unwrap_or_default();
                let link = item["url"].as_str().unwrap_or_default();
                let user_name = item["user"]["name"].as_str().unwrap_or_default();

                let mut result = SearchResult::new(
                    title.to_string(),
                    link.to_string(),
                    format!("by {}", user_name),
                    "mixcloud".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::Music);
                result.thumbnail = item["pictures"]["medium"]
                    .as_str()
                    .map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
