//! Yep — privacy-focused search engine.
//!
//! Queries the Yep JSON API for web results.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use smallvec::smallvec;

pub struct Yep {
    metadata: EngineMetadata,
    client: Client,
}

impl Yep {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "yep".to_string().into(),
                display_name: "Yep".to_string().into(),
                homepage: "https://yep.com".to_string().into(),
                categories: smallvec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Yep {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://api.yep.com/fs/2/search?client=web&no_correct=false&q={}&safeSearch=moderate&type=web",
            urlencoding::encode(&query.query)
        );

        let resp = self
            .client
            .get(&url)
            .header("Referer", "https://yep.com/")
            .header("Origin", "https://yep.com")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Handle non-JSON / empty responses
        if text.trim().is_empty() || text.starts_with("<!") || text.starts_with("<html") {
            return Ok(Vec::new());
        }

        let resp: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Yep returns an array: [metadata, {results: [...]}]
        let empty = vec![];
        let items = resp
            .get(1)
            .and_then(|v| v.get("results"))
            .and_then(|v| v.as_array())
            // Fallback: try 'results' at top level
            .or_else(|| resp.get("results").and_then(|v| v.as_array()))
            .unwrap_or(&empty);

        let mut results = Vec::new();
        let mut rank = 0u32;

        for item in items {
            let result_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
            if result_type != "Organic" {
                continue;
            }

            rank += 1;
            let title = item
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let page_url = item
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let snippet_raw = item.get("snippet").and_then(|v| v.as_str()).unwrap_or("");
            let snippet = html_escape::decode_html_entities(snippet_raw).to_string();

            let mut r = SearchResult::new(title, page_url, snippet, self.metadata.name.clone());
            r.engine_rank = rank;
            results.push(r);
        }

        Ok(results)
    }
}
