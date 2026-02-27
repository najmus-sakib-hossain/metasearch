//! BitChute video search engine.
//!
//! BitChute is a peer-to-peer video hosting service. This engine
//! queries their JSON search API.

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;
use metasearch_core::error::{MetasearchError, Result};
use metasearch_core::category::SearchCategory;

const API_URL: &str = "https://api.bitchute.com/api/beta/search/videos";
const RESULTS_PER_PAGE: u32 = 20;

pub struct BitChute {
    metadata: EngineMetadata,
    client: Client,
}

impl BitChute {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "bitchute".to_string(),
                display_name: "BitChute".to_string(),
                homepage: "https://www.bitchute.com".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 3000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    videos: Option<Vec<VideoItem>>,
}

#[derive(Deserialize)]
struct VideoItem {
    video_name: Option<String>,
    video_id: Option<String>,
    description: Option<String>,
    #[serde(default)]
    channel: ChannelInfo,
}

#[derive(Deserialize, Default)]
struct ChannelInfo {
    channel_name: Option<String>,
}

#[async_trait]
impl SearchEngine for BitChute {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let offset = query.page.saturating_sub(1) * RESULTS_PER_PAGE;

        let body = json!({
            "query": query.query,
            "offset": offset,
            "limit": RESULTS_PER_PAGE,
            "sensitivity_id": "normal",
            "sort": "new"
        });

        let resp = self
            .client
            .post(API_URL)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?
            .json::<ApiResponse>()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        for item in resp.videos.unwrap_or_default() {
            let video_id = item.video_id.unwrap_or_default();
            let title = item.video_name.unwrap_or_default();
            if title.is_empty() || video_id.is_empty() {
                continue;
            }

            let video_url = format!("https://www.bitchute.com/video/{}", video_id);

            let mut content = html_escape::decode_html_entities(
                &item.description.unwrap_or_default(),
            )
            .to_string();

            // Strip HTML tags from description
            let tag_re = regex::Regex::new(r"<[^>]+>").unwrap();
            content = tag_re.replace_all(&content, "").trim().to_string();

            if let Some(channel) = item.channel.channel_name.as_deref() {
                if !content.is_empty() {
                    content = format!("{} — {}", channel, content);
                } else {
                    content = channel.to_string();
                }
            }

            results.push(SearchResult::new(title, video_url, content, "bitchute".to_string()));
        }

        Ok(results)
    }
}
