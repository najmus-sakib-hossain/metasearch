//! Searchcode Code engine — search source code via Searchcode API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct SearchcodeCode {
    metadata: EngineMetadata,
    client: Client,
}

impl SearchcodeCode {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "searchcode_code".to_string(),
                display_name: "Searchcode".to_string(),
                homepage: "https://searchcode.com".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for SearchcodeCode {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://searchcode.com/api/codesearch_I/?q={}",
            urlencoding::encode(&query.query),
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(Vec::new());
        }

        let text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        if text.trim_start().starts_with('<') {
            return Ok(Vec::new());
        }

        let data: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => return Ok(Vec::new()),
        };

        let mut results = Vec::new();

        if let Some(items) = data["results"].as_array() {
            for (i, item) in items.iter().enumerate() {
                let item_url = item["url"].as_str().unwrap_or_default();
                let name = item["name"].as_str().unwrap_or_default();
                let filename = item["filename"].as_str().unwrap_or("");
                let repo = item["repo"].as_str().unwrap_or("");

                if item_url.is_empty() {
                    continue;
                }

                let title = if filename.is_empty() {
                    name.to_string()
                } else {
                    format!("{} - {}", name, filename)
                };

                let mut result = SearchResult::new(
                    title,
                    item_url.to_string(),
                    repo.to_string(),
                    "searchcode_code".to_string(),
                );
                result.engine_rank = (i + 1) as u32;
                result.category = SearchCategory::IT.to_string();
                results.push(result);
            }
        }

        Ok(results)
    }
}
