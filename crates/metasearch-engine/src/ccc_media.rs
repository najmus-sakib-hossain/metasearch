//! CCC Media (media.ccc.de) video search engine.
//!
//! Queries the media.ccc.de API for Chaos Computer Club conference recordings.

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct CccMedia {
    metadata: EngineMetadata,
    client: Client,
}

impl CccMedia {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "ccc_media".to_string(),
                display_name: "CCC Media".to_string(),
                homepage: "https://media.ccc.de".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 5000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Event {
    title: String,
    description: Option<String>,
    frontend_link: Option<String>,
    thumb_url: Option<String>,
    #[serde(default)]
    recordings: Vec<Recording>,
}

#[derive(Deserialize)]
struct Recording {
    mime_type: Option<String>,
    recording_url: Option<String>,
}

#[async_trait]
impl SearchEngine for CccMedia {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let url = format!(
            "https://api.media.ccc.de/public/events/search?q={}&page={}",
            urlencoding::encode(&query.query),
            page
        );

        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let api_resp: ApiResponse = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        for event in api_resp.events {
            let link = event.frontend_link.as_deref().unwrap_or_default();
            if link.is_empty() {
                continue;
            }

            let description = event.description.as_deref().unwrap_or("");
            // Truncate long descriptions
            let snippet = if description.len() > 300 {
                format!("{}...", &description[..300])
            } else {
                description.to_string()
            };

            let mut result = SearchResult::new(&event.title, link, &snippet, "ccc_media");
            result.category = SearchCategory::Videos.to_string();

            if let Some(thumb) = &event.thumb_url {
                result.thumbnail = Some(thumb.clone());
            }

            results.push(result);
        }

        Ok(results)
    }
}
