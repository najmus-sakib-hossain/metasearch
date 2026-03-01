//! DuckDuckGo Extra — image search via DuckDuckGo's i.js endpoint.
//!
//! Fetches a VQD token from the DuckDuckGo homepage, then queries the
//! JSON image-search API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct DuckDuckGoExtra {
    metadata: EngineMetadata,
    client: Client,
}

impl DuckDuckGoExtra {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "duckduckgo_extra".to_string(),
                display_name: "DuckDuckGo Images".to_string(),
                homepage: "https://duckduckgo.com".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 8000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for DuckDuckGoExtra {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let encoded = urlencoding::encode(&query.query);

        // Step 1: Obtain VQD token from the DuckDuckGo search page.
        let token_url = format!("https://duckduckgo.com/?q={}", encoded);
        let token_resp = self
            .client
            .get(&token_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let token_html = token_resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Extract vqd token using simple string search (handles both quote styles)
        let vqd = {
            // Try double-quote: vqd="..."
            let dq = token_html
                .find("vqd=\"")
                .map(|i| {
                    let start = i + 5;
                    let rest = &token_html[start..];
                    let end = rest.find('"').unwrap_or(0);
                    rest[..end].to_string()
                })
                .filter(|s| !s.is_empty());

            if let Some(v) = dq {
                v
            } else {
                // Try single-quote: vqd='...'
                let sq = token_html
                    .find("vqd='")
                    .map(|i| {
                        let start = i + 5;
                        let rest = &token_html[start..];
                        let end = rest.find('\'').unwrap_or(0);
                        rest[..end].to_string()
                    })
                    .filter(|s| !s.is_empty());

                match sq {
                    Some(v) => v,
                    None => {
                        return Err(MetasearchError::ParseError(
                            "Could not extract VQD token from DuckDuckGo".to_string(),
                        ));
                    }
                }
            }
        };

        // Step 2: Fetch image results with the VQD token
        let offset = (query.page.saturating_sub(1)) * 100;
        let search_url = format!(
            "https://duckduckgo.com/i.js?o=json&q={}&u=bing&l=us-en&s={}&vqd={}",
            encoded, offset, vqd,
        );

        let resp = self
            .client
            .get(&search_url)
            .header("Referer", "https://duckduckgo.com/")
            .header("Cookie", "p=-2")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let items = match json.get("results").and_then(|r| r.as_array()) {
            Some(arr) => arr,
            None => return Ok(Vec::new()),
        };

        let mut results = Vec::new();

        for (i, item) in items.iter().enumerate() {
            let url = item["url"].as_str().unwrap_or_default();
            if url.is_empty() {
                continue;
            }

            let title = item["title"].as_str().unwrap_or("Untitled").to_string();
            let image = item["image"].as_str().unwrap_or_default();
            let width = item["width"].as_u64().unwrap_or(0);
            let height = item["height"].as_u64().unwrap_or(0);
            let source = item["source"].as_str().unwrap_or_default();

            let content = if !source.is_empty() {
                format!("{}x{} — {}", width, height, source)
            } else {
                format!("{}x{}", width, height)
            };

            let mut result = SearchResult::new(&title, url, &content, "duckduckgo_extra");
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::Images.to_string();
            if !image.is_empty() {
                result.thumbnail = Some(image.to_string());
            }
            results.push(result);
        }

        Ok(results)
    }
}
