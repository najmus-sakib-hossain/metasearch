//! Qwant engine — search via Qwant API (JSON).
//! Translated from SearXNG `searx/engines/qwant.py`.
//!
//! Implements the `web` category using Qwant's undocumented v3 API.
//! The API returns JSON with results in `data.result.items.mainline`.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct Qwant {
    metadata: EngineMetadata,
    client: Client,
}

impl Qwant {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "qwant".to_string(),
                display_name: "Qwant".to_string(),
                homepage: "https://www.qwant.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Qwant {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let count: u32 = 10;
        let offset = (page - 1) * count;

        let url = format!(
            "https://api.qwant.com/v3/search/web?q={}&count={}&locale=en_US&safesearch=1&llm=false&tgp=3&offset={}",
            urlencoding::encode(&query.query),
            count,
            offset,
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Check API status
        if data["status"].as_str() != Some("success") {
            let error_code = data["data"]["error_code"].as_u64().unwrap_or(0);
            return Err(MetasearchError::Other(format!(
                "Qwant API error (code: {})",
                error_code
            )));
        }

        let mut results = Vec::new();

        // Navigate: data -> result -> items -> mainline
        let mainline = &data["data"]["result"]["items"]["mainline"];

        if let Some(rows) = mainline.as_array() {
            for row in rows {
                let row_type = row["type"].as_str().unwrap_or("");

                // Only process 'web' type results, skip ads
                if row_type != "web" {
                    continue;
                }

                if let Some(items) = row["items"].as_array() {
                    for item in items {
                        let title = item["title"].as_str().unwrap_or_default();
                        let item_url = item["url"].as_str().unwrap_or_default();
                        let desc = item["desc"].as_str().unwrap_or("");

                        if title.is_empty() || item_url.is_empty() {
                            continue;
                        }

                        let mut sr = SearchResult::new(
                            title.to_string(),
                            item_url.to_string(),
                            desc.to_string(),
                            "qwant".to_string(),
                        );
                        sr.engine_rank = (results.len() + 1) as u32;
                        sr.category = SearchCategory::General.to_string();
                        results.push(sr);
                    }
                }
            }
        }

        Ok(results)
    }
}
