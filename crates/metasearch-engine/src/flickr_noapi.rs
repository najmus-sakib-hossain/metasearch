//! Flickr (no API) — image search on Flickr without using the official API.
//!
//! Scrapes the Flickr search page and extracts photo links from the HTML.
//!
//! Reference: <https://www.flickr.com>

use std::collections::HashSet;

use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

use metasearch_core::category::SearchCategory;
use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::error::MetasearchError;
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;

const BASE_URL: &str = "https://www.flickr.com";

pub struct FlickrNoapi {
    metadata: EngineMetadata,
    client: Client,
}

impl FlickrNoapi {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "flickr_noapi".to_string(),
                display_name: "Flickr (no API)".to_string(),
                homepage: BASE_URL.to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.7,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for FlickrNoapi {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "{}/search/?text={}&page={}",
            BASE_URL,
            urlencoding::encode(&query.query),
            query.page
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let mut results = Vec::new();

        // Extract photo page links with titles from the search results HTML.
        // Flickr photo pages have URLs like /photos/{user}/{photo_id}/
        let re = Regex::new(r#"<a[^>]+href="(/photos/[^/]+/\d+/)"[^>]*title="([^"]*)""#)
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut seen = HashSet::new();
        for cap in re.captures_iter(&body) {
            let href = cap[1].to_string();
            if !seen.insert(href.clone()) {
                continue;
            }
            let title_raw = &cap[2];
            let title = html_escape::decode_html_entities(title_raw).to_string();
            let photo_url = format!("{}{}", BASE_URL, href);

            let rank = results.len();
            let mut result = SearchResult::new(
                if title.is_empty() {
                    "Flickr photo".to_string()
                } else {
                    title
                },
                photo_url,
                String::new(),
                "flickr_noapi",
            );
            result.engine_rank = (rank + 1) as u32;
            results.push(result);
        }

        Ok(results)
    }
}
