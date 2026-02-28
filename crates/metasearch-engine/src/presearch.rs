//! Presearch search engine implementation.
//! Two-step process: get searchId from HTML, then fetch JSON results.
//! URL: https://presearch.com/search
//! Features: Paging

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use regex::Regex;
use reqwest::Client;
use tracing::info;

pub struct Presearch {
    metadata: EngineMetadata,
    client: Client,
}

impl Presearch {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "presearch".to_string(),
                display_name: "Presearch".to_string(),
                homepage: "https://presearch.io".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 8000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Presearch {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page.max(1);

        // Step 1: GET search page to extract searchId
        let search_url = format!(
            "https://presearch.com/search?q={}&page={}",
            urlencoding::encode(&query.query),
            page,
        );

        let resp = self
            .client
            .get(&search_url)
            .header(
                "Cookie",
                "b=1; presearch_session=; use_local_search_results=false",
            )
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Extract searchId from HTML
        let id_re = Regex::new(r#"window\.searchId\s*=\s*"([^"]+)""#)
            .map_err(|e| MetasearchError::ParseError(format!("Regex error: {}", e)))?;

        let search_id = match id_re.captures(&html_text) {
            Some(caps) => match caps.get(1) {
                Some(m) => m.as_str().to_string(),
                None => return Ok(Vec::new()),
            },
            None => return Ok(Vec::new()),
        };

        // Step 2: Fetch JSON results using searchId
        let results_url = format!(
            "https://presearch.com/results?id={}",
            urlencoding::encode(&search_id),
        );

        let resp = self
            .client
            .get(&results_url)
            .header(
                "Cookie",
                "b=1; presearch_session=; use_local_search_results=false",
            )
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
            .map_err(|e| MetasearchError::ParseError(format!("JSON error: {}", e)))?;

        let mut results = Vec::new();

        let items = match data.get("standardResults").and_then(|s| s.as_array()) {
            Some(items) => items,
            None => return Ok(results),
        };

        for (i, item) in items.iter().enumerate() {
            let link = item["link"].as_str().unwrap_or_default();
            let title = item["title"].as_str().unwrap_or_default();
            let description = item["description"].as_str().unwrap_or_default();

            if title.is_empty() || link.is_empty() {
                continue;
            }

            let mut r = SearchResult::new(title, link, description, "presearch");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        info!(
            engine = "presearch",
            count = results.len(),
            "Search complete"
        );
        Ok(results)
    }
}
