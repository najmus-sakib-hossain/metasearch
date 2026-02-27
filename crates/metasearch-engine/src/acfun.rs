//! AcFun (acfun.cn) video search engine.
//!
//! Searches AcFun, a Chinese video hosting platform, by scraping
//! embedded JSON data from the HTML search results page.

use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;
use metasearch_core::error::MetasearchError;
use metasearch_core::category::SearchCategory;

pub struct AcFun {
    client: Client,
}

impl AcFun {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[derive(Deserialize)]
struct BigPipePayload {
    html: Option<String>,
}

#[derive(Deserialize)]
struct VideoExposureLog {
    content_id: Option<String>,
    title: Option<String>,
}

#[async_trait]
impl SearchEngine for AcFun {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "AcFun".to_string(),
            base_url: "https://www.acfun.cn".to_string(),
            categories: vec![SearchCategory::Videos],
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let mut url = Url::parse("https://www.acfun.cn/search").map_err(|e| MetasearchError::Request(e.to_string()))?;
        url.query_pairs_mut()
            .append_pair("keyword", &query.query)
            .append_pair("pCursor", &query.page.unwrap_or(1).to_string());

        let resp = self
            .client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| MetasearchError::Request(e.to_string()))?
            .text()
            .await
            .map_err(|e| MetasearchError::Request(e.to_string()))?;

        let mut results = Vec::new();
        let re = Regex::new(r"bigPipe\.onPageletArrive\((\{.*?\})\);")
            .map_err(|e| MetasearchError::Parse(e.to_string()))?;

        for cap in re.captures_iter(&resp) {
            let json_str = &cap[1];
            let payload: BigPipePayload = match serde_json::from_str(json_str) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let raw_html = match payload.html {
                Some(h) if !h.is_empty() => h,
                _ => continue,
            };

            let document = scraper::Html::parse_document(&raw_html);
            let video_sel = scraper::Selector::parse("div.search-video").unwrap();

            for block in document.select(&video_sel) {
                if let Some(log_attr) = block.value().attr("data-exposure-log") {
                    if let Ok(log) = serde_json::from_str::<VideoExposureLog>(log_attr) {
                        let content_id = log.content_id.unwrap_or_default();
                        let title = log.title.unwrap_or_default();
                        if title.is_empty() || content_id.is_empty() {
                            continue;
                        }

                        let video_url = format!("https://www.acfun.cn/v/ac{}", content_id);

                        results.push(SearchResult {
                            title,
                            url: video_url,
                            content: String::new(),
                            engine: "AcFun".to_string(),
                            score: 1.0,
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}
