//! Bitchute engine — search videos via Bitchute JSON API.
//! Translated from SearXNG `searx/engines/bitchute.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Bitchute {
    client: Client,
}

impl Bitchute {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Bitchute {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "bitchute".to_string(),
            display_name: "BitChute".to_string(),
            categories: vec![SearchCategory::Videos],
            enabled: true,
            weight: 0.5,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let results_per_page: u32 = 20;
        let offset = (page as u32 - 1) * results_per_page;

        let body = serde_json::json!({
            "offset": offset,
            "limit": results_per_page,
            "query": query.query,
            "sensitivity_id": "normal",
            "sort": "new"
        });

        let resp = self.client
            .post("https://api.bitchute.com/api/beta/search/videos")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(videos) = data["videos"].as_array() {
            for (i, item) in videos.iter().enumerate() {
                let video_id = item["video_id"].as_str().unwrap_or_default();
                let title = item["video_name"].as_str().unwrap_or_default();
                let description = item["description"].as_str().unwrap_or_default();
                let channel = item["channel"]["channel_name"].as_str().unwrap_or_default();

                let content = html_escape::decode_html_entities(description).to_string();
                let snippet = if content.len() > 200 {
                    format!("{}... — {}", &content[..200], channel)
                } else {
                    format!("{} — {}", content, channel)
                };

                let video_url = format!("https://www.bitchute.com/video/{}", video_id);

                let mut result = SearchResult::new(
                    title.to_string(),
                    video_url,
                    snippet,
                    "bitchute".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::Videos);
                result.thumbnail = item["thumbnail_url"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
