//! Hugging Face engine — search models, datasets, spaces via HF API.
//! Translated from SearXNG `searx/engines/huggingface.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct HuggingFace {
    metadata: EngineMetadata,
    client: Client,
    endpoint: String, // "models", "datasets", or "spaces"
}

impl HuggingFace {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "huggingface".to_string(),
                display_name: "Hugging Face".to_string(),
                homepage: "https://huggingface.co".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
            endpoint: "models".to_string(),
        }
    }

    pub fn with_endpoint(client: Client, endpoint: &str) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "huggingface".to_string(),
                display_name: "Hugging Face".to_string(),
                homepage: "https://huggingface.co".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
            endpoint: endpoint.to_string(),
        }
    }
}

#[async_trait]
impl SearchEngine for HuggingFace {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://huggingface.co/api/{}?direction=-1&search={}",
            self.endpoint,
            urlencoding::encode(&query.query),
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(entries) = data.as_array() {
            for (i, entry) in entries.iter().take(25).enumerate() {
                let id = entry["id"].as_str().unwrap_or_default();

                let entry_url = if self.endpoint != "models" {
                    format!("https://huggingface.co/{}/{}", self.endpoint, id)
                } else {
                    format!("https://huggingface.co/{}", id)
                };

                let likes = entry["likes"].as_u64().unwrap_or(0);
                let downloads = entry["downloads"].as_u64().unwrap_or(0);

                let tags: Vec<String> = entry["tags"].as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .take(5)
                    .filter_map(|t| t.as_str().map(|s| s.to_string()))
                    .collect();

                let description = entry["description"].as_str().unwrap_or("");

                let mut content_parts = Vec::new();
                if likes > 0 {
                    content_parts.push(format!("❤️ {}", likes));
                }
                if downloads > 0 {
                    content_parts.push(format!("📥 {}", downloads));
                }
                if !tags.is_empty() {
                    content_parts.push(format!("Tags: {}", tags.join(", ")));
                }
                if !description.is_empty() {
                    content_parts.push(description.to_string());
                }

                let snippet = content_parts.join(" | ");

                let mut result = SearchResult::new(
                    id.to_string(),
                    entry_url,
                    snippet,
                    "huggingface".to_string(),
                );
                result.engine_rank = (i + 1) as u32;
                result.category = SearchCategory::IT.to_string();
                results.push(result);
            }
        }

        Ok(results)
    }
}
