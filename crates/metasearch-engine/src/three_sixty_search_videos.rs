//! 360 Search Videos — Chinese video search from 360kan.
//!
//! Simple JSON API, no authentication required.
//!
//! Reference: <https://tv.360kan.com>

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde::Deserialize;

pub struct ThreeSixtySearchVideos {
    metadata: EngineMetadata,
    client: Client,
}

impl ThreeSixtySearchVideos {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "360search_videos".to_string(),
                display_name: "360 Search Videos".to_string(),
                homepage: "https://tv.360kan.com".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.5,
            },
            client,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Option<ApiData>,
}

#[derive(Debug, Deserialize)]
struct ApiData {
    result: Option<Vec<VideoItem>>,
}

#[derive(Debug, Deserialize)]
struct VideoItem {
    title: Option<String>,
    play_url: Option<String>,
    description: Option<String>,
    cover_img: Option<String>,
}

#[async_trait]
impl SearchEngine for ThreeSixtySearchVideos {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let start = query.page.saturating_sub(1) * 10;

        let url = format!(
            "https://tv.360kan.com/v1/video/list?count=10&q={}&start={}",
            urlencoding::encode(&query.query),
            start,
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
            )
            .header("Accept", "application/json, */*")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Referer", "https://tv.360kan.com/")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(Vec::new());
        }

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let items = api.data.and_then(|d| d.result).unwrap_or_default();

        let mut results = Vec::new();

        for (rank, item) in items.iter().enumerate() {
            let title = match &item.title {
                Some(t) if !t.is_empty() => t.clone(),
                _ => continue,
            };
            let play_url = match &item.play_url {
                Some(u) if !u.is_empty() => u.clone(),
                _ => continue,
            };
            let description = item.description.clone().unwrap_or_default();

            let mut result =
                SearchResult::new(title, play_url, description, self.metadata.name.clone());
            result.engine_rank = (rank + 1) as u32;
            result.thumbnail = item.cover_img.clone();
            results.push(result);
        }

        Ok(results)
    }
}
