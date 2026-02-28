//! Adobe Stock — royalty-free image/asset search via JSON API.
//!
//! Reference: <https://stock.adobe.com/>

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct AdobeStock {
    metadata: EngineMetadata,
    client: Client,
}

impl AdobeStock {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "adobe_stock".to_string(),
                display_name: "Adobe Stock".to_string(),
                homepage: "https://stock.adobe.com".to_string(),
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
impl SearchEngine for AdobeStock {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page;
        let url = format!(
            "https://stock.adobe.com/de/Ajax/Search?k={}&limit=10&order=relevance&search_page={}&search_type=pagination&filters%5Bcontent_type%3Aphoto%5D=1&filters%5Bcontent_type%3Aillustration%5D=1&filters%5Bcontent_type%3Azip_vector%5D=1&filters%5Bcontent_type%3Aimage%5D=1&filters%5Bcontent_type%3Avideo%5D=0&filters%5Bcontent_type%3Atemplate%5D=0&filters%5Bcontent_type%3A3d%5D=0&filters%5Bcontent_type%3Aaudio%5D=0",
            urlencoding::encode(&query.query),
            page
        );

        let resp = self
            .client
            .get(&url)
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let json: serde_json::Value = resp
            .json::<serde_json::Value>()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(items) = json.get("items").and_then(|v| v.as_object()) {
            for (_key, item) in items {
                let title = item
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let content_url = item
                    .get("content_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let asset_type = item
                    .get("asset_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let thumbnail = item
                    .get("thumbnail_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let author = item
                    .get("author")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                if content_url.is_empty() {
                    continue;
                }

                let snippet = if author.is_empty() {
                    asset_type.to_string()
                } else {
                    format!("{} — by {}", asset_type, author)
                };

                let mut result = SearchResult::new(
                    title.to_string(),
                    content_url.to_string(),
                    snippet,
                    "Adobe Stock".to_string(),
                );
                result.category = SearchCategory::Images.to_string();
                if !thumbnail.is_empty() {
                    result.thumbnail = Some(thumbnail.to_string());
                }
                results.push(result);
            }
        }

        Ok(results)
    }
}
