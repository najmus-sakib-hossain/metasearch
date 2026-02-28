//! Piped (alternative YouTube frontend) video search — configurable instance URLs.
//! SearXNG equivalent: `piped.py`
//!
//! Piped consists of a backend (API) and frontend (UI). Both URLs must be
//! configured. The backend URL is used for API calls, the frontend URL
//! is used for constructing user-facing links.

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};

pub struct Piped {
    client: Client,
    backend_url: String,
    frontend_url: String,
}

impl Piped {
    pub fn new(client: Client, backend_url: &str, frontend_url: &str) -> Self {
        Self {
            client,
            backend_url: backend_url.trim_end_matches('/').to_string(),
            frontend_url: frontend_url.trim_end_matches('/').to_string(),
        }
    }
}

#[derive(Deserialize)]
struct PipedResponse {
    items: Vec<PipedItem>,
    nextpage: Option<String>,
}

#[derive(Deserialize)]
struct PipedItem {
    url: Option<String>,
    title: Option<String>,
    #[serde(rename = "shortDescription")]
    short_description: Option<String>,
    thumbnail: Option<String>,
    views: Option<i64>,
    duration: Option<i64>,
    #[serde(rename = "uploaderName")]
    uploader_name: Option<String>,
}

#[async_trait]
impl SearchEngine for Piped {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "Piped".to_string(),
            description: "Piped (alt YouTube frontend) video search — configurable instance URLs"
                .to_string(),
            categories: vec![metasearch_core::category::SearchCategory::Videos],
            enabled: !self.backend_url.is_empty(),
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        if self.backend_url.is_empty() {
            return Ok(Vec::new());
        }

        let url = format!(
            "{}/search?q={}&filter=videos",
            self.backend_url,
            urlencoding::encode(&query.query),
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::EngineError(format!("Piped: {e}")))?;

        let data: PipedResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::EngineError(format!("Piped JSON: {e}")))?;

        let frontend = if self.frontend_url.is_empty() {
            "https://piped.video"
        } else {
            &self.frontend_url
        };

        let mut results = Vec::new();
        for (i, item) in data.items.iter().enumerate() {
            let path = match &item.url {
                Some(u) if !u.is_empty() => u,
                _ => continue,
            };
            let title = item.title.clone().unwrap_or_default();
            let video_url = format!("{}{}", frontend, path);
            let content = item.short_description.clone().unwrap_or_default();

            results.push(SearchResult {
                title,
                url: video_url,
                content,
                engine: "Piped".to_string(),
                engine_rank: (i + 1) as u32,
            });
        }
        Ok(results)
    }
}
