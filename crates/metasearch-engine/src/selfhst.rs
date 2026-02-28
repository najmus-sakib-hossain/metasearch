//! Selfh.st engine — search self-hosted application icons via selfh.st index.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct Selfhst {
    metadata: EngineMetadata,
    client: Client,
}

impl Selfhst {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "selfhst".to_string(),
                display_name: "Selfh.st Icons".to_string(),
                homepage: "https://selfh.st/icons/".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Selfhst {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = "https://cdn.jsdelivr.net/gh/selfhst/icons/index.json";

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let items = match data.as_array() {
            Some(arr) => arr,
            None => return Ok(Vec::new()),
        };

        let query_lower = query.query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        let mut results = Vec::new();

        for item in items {
            let reference = item["Reference"].as_str().unwrap_or_default();
            let ref_lower = reference.to_lowercase();

            let matches = query_words.iter().all(|word| ref_lower.contains(word));
            if !matches {
                continue;
            }

            let name = item["Name"].as_str().unwrap_or(reference);

            // Determine best format for image URL
            let format = if item["SVG"].as_bool().unwrap_or(false) {
                "svg"
            } else if item["PNG"].as_bool().unwrap_or(false) {
                "png"
            } else if item["WebP"].as_bool().unwrap_or(false) {
                "webp"
            } else {
                continue;
            };

            let image_url = format!(
                "https://cdn.jsdelivr.net/gh/selfhst/icons/{}/{}.{}",
                format, reference, format,
            );

            let mut result = SearchResult::new(
                name.to_string(),
                "https://selfh.st/icons/".to_string(),
                format!("Self-hosted icon: {}", reference),
                "selfhst".to_string(),
            );
            result.engine_rank = (results.len() + 1) as u32;
            result.category = SearchCategory::Images.to_string();
            result.thumbnail = Some(image_url);
            results.push(result);

            if results.len() >= 20 {
                break;
            }
        }

        Ok(results)
    }
}
