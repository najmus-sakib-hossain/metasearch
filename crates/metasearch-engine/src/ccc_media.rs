//! CCC Media engine — search media.ccc.de (Chaos Computer Club recordings).
//! Translated from SearXNG `searx/engines/ccc_media.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct CccMedia {
    client: Client,
}

impl CccMedia {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for CccMedia {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "ccc_media".to_string(),
            display_name: "media.ccc.de".to_string(),
            categories: vec![SearchCategory::Videos],
            enabled: true,
            weight: 0.6,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);

        let url = format!(
            "https://api.media.ccc.de/public/events/search?q={}&page={}",
            urlencoding::encode(&query.query),
            page,
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(events) = data["events"].as_array() {
            for (i, item) in events.iter().enumerate() {
                let title = item["title"].as_str().unwrap_or_default();
                let link = item["frontend_link"].as_str().unwrap_or_default();
                let description = item["description"].as_str().unwrap_or_default();
                let thumbnail = item["thumb_url"].as_str();

                // Find a video recording URL (prefer mp4)
                let iframe_src = item["recordings"].as_array().and_then(|recs| {
                    let mut fallback: Option<&str> = None;
                    for rec in recs {
                        let mime = rec["mime_type"].as_str().unwrap_or_default();
                        if mime.starts_with("video") {
                            let rec_url = rec["recording_url"].as_str().unwrap_or_default();
                            if mime == "video/mp4" {
                                return Some(rec_url.to_string());
                            }
                            if fallback.is_none() {
                                fallback = Some(rec_url);
                            }
                        }
                    }
                    fallback.map(|s| s.to_string())
                });

                let length_secs = item["length"].as_u64().unwrap_or(0);
                let duration = if length_secs > 0 {
                    let h = length_secs / 3600;
                    let m = (length_secs % 3600) / 60;
                    let s = length_secs % 60;
                    if h > 0 {
                        format!("{}h{}m{}s", h, m, s)
                    } else {
                        format!("{}m{}s", m, s)
                    }
                } else {
                    String::new()
                };

                let snippet = if duration.is_empty() {
                    description.chars().take(300).collect()
                } else {
                    format!("[{}] {}", duration, description.chars().take(280).collect::<String>())
                };

                let mut result = SearchResult::new(
                    title.to_string(),
                    link.to_string(),
                    snippet,
                    "ccc_media".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::Videos);
                result.thumbnail = thumbnail.map(|s| s.to_string());
                if let Some(ref src) = iframe_src {
                    result.iframe_src = Some(src.clone());
                }
                results.push(result);
            }
        }

        Ok(results)
    }
}
