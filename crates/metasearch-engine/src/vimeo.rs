//! Vimeo engine — search videos via Vimeo HTML (embedded JSON).
//! Translated from SearXNG `searx/engines/vimeo.py`.
//!
//! Vimeo embeds search data as `var data = {...};` in the HTML.
//! We extract that JSON and parse `filtered.data` for video results.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Vimeo {
    client: Client,
}

impl Vimeo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

/// Extract a substring between `start` and `end` markers.
fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_idx = text.find(start)? + start.len();
    let remaining = &text[start_idx..];
    let end_idx = remaining.find(end)?;
    Some(&remaining[..end_idx])
}

#[async_trait]
impl SearchEngine for Vimeo {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "vimeo".to_string(),
            display_name: "Vimeo".to_string(),
            categories: vec![SearchCategory::Videos],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);

        let url = format!(
            "https://vimeo.com/search/page:{}?q={}",
            page,
            urlencoding::encode(&query.query),
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Extract embedded JSON: `var data = {...};\n`
        let json_str = match extract_between(&html_text, "var data = ", ";\n") {
            Some(s) => s,
            None => return Ok(Vec::new()),
        };

        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        // Navigate: filtered -> data (array of results)
        if let Some(items) = data["filtered"]["data"].as_array() {
            for (i, item) in items.iter().enumerate() {
                // Each item has a "type" key, and the actual data is under that key
                let item_type = item["type"].as_str().unwrap_or("clip");
                let inner = &item[item_type];

                if inner.is_null() {
                    continue;
                }

                // Extract video ID from the URI (e.g. "/videos/123456")
                let uri = inner["uri"].as_str().unwrap_or_default();
                let video_id = uri.rsplit('/').next().unwrap_or("");
                if video_id.is_empty() {
                    continue;
                }

                let video_url = format!("https://vimeo.com/{}", video_id);
                let title = inner["name"].as_str().unwrap_or("Untitled").to_string();

                // Get the largest thumbnail
                let thumbnail = inner["pictures"]["sizes"].as_array()
                    .and_then(|sizes| sizes.last())
                    .and_then(|s| s["link"].as_str())
                    .map(|s| s.to_string());

                let iframe_src = format!("https://player.vimeo.com/video/{}", video_id);

                let snippet = format!(
                    "Vimeo video — embed: {}",
                    iframe_src,
                );

                let mut sr = SearchResult::new(
                    title,
                    video_url,
                    snippet,
                    "vimeo".to_string(),
                );
                sr.engine_rank = Some(i + 1);
                sr.category = Some(SearchCategory::Videos);
                sr.thumbnail = thumbnail;
                results.push(sr);
            }
        }

        Ok(results)
    }
}
