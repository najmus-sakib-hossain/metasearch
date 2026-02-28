//! DeviantArt — image search on DeviantArt.
//!
//! Scrapes HTML search results from deviantart.com.
//!
//! Reference: <https://www.deviantart.com>

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};

pub struct DeviantArt {
    metadata: EngineMetadata,
    client: Client,
}

impl DeviantArt {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "deviantart".to_string(),
                display_name: "DeviantArt".to_string(),
                homepage: "https://www.deviantart.com".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.7,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for DeviantArt {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://www.deviantart.com/search?q={}",
            urlencoding::encode(&query.query),
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let result_sel = Selector::parse("a[data-hook='deviation_link']")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let img_sel = Selector::parse("img")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (rank, item) in document.select(&result_sel).enumerate() {
            let href = item.value().attr("href").unwrap_or_default();
            if href.is_empty() || !href.starts_with("http") {
                continue;
            }

            let title = item
                .value()
                .attr("aria-label")
                .unwrap_or("DeviantArt Image")
                .to_string();

            let img_el = item.select(&img_sel).next();
            let thumbnail_src = img_el
                .and_then(|el| el.value().attr("src"))
                .unwrap_or_default()
                .to_string();

            if thumbnail_src.is_empty() {
                continue;
            }

            let mut result = SearchResult::new(
                title,
                href.to_string(),
                String::new(),
                self.metadata.name.clone(),
            );
            result.engine_rank = (rank + 1) as u32;
            result.thumbnail = Some(thumbnail_src);
            results.push(result);
        }

        Ok(results)
    }
}
