//! Deezer engine — search music tracks via Deezer API.
//! Translated from SearXNG `searx/engines/deezer.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Deezer {
    client: Client,
}

impl Deezer {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Deezer {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "deezer".to_string(),
            display_name: "Deezer".to_string(),
            categories: vec![SearchCategory::Music],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let offset = (page as u32 - 1) * 25;

        let url = format!(
            "https://api.deezer.com/search?q={}&index={}",
            urlencoding::encode(&query.query),
            offset,
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(items) = data["data"].as_array() {
            for (i, item) in items.iter().enumerate() {
                if item["type"].as_str() != Some("track") {
                    continue;
                }

                let title = item["title"].as_str().unwrap_or_default();
                let mut link = item["link"].as_str().unwrap_or_default().to_string();
                if link.starts_with("http://") {
                    link = format!("https{}", &link[4..]);
                }

                let artist = item["artist"]["name"].as_str().unwrap_or("");
                let album = item["album"]["title"].as_str().unwrap_or("");
                let duration = item["duration"].as_u64().unwrap_or(0);
                let minutes = duration / 60;
                let seconds = duration % 60;

                let snippet = format!(
                    "{} — {} [{}:{:02}]",
                    artist, album, minutes, seconds,
                );

                let mut result = SearchResult::new(
                    title.to_string(),
                    link,
                    snippet,
                    "deezer".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::Music);
                result.thumbnail = item["album"]["cover_medium"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
