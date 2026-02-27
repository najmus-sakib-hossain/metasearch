//! DeviantArt engine — search images on DeviantArt.
//! Translated from SearXNG `searx/engines/deviantart.py`.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::{MetasearchError, Result},
};

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
                timeout_ms: 3000,
                weight: 0.7,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for DeviantArt {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://www.deviantart.com/search?q={}",
            urlencoding::encode(&query.query),
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let result_sel = Selector::parse("div[class*='V_S0t_'] div div a, a[data-hook='deviation_link']").unwrap();
        let img_sel = Selector::parse("div img, img").unwrap();

        let mut results = Vec::new();

        for (i, item) in document.select(&result_sel).enumerate() {
            let href = item.value().attr("href").unwrap_or_default();
            if href.is_empty() || !href.starts_with("http") {
                continue;
            }

            let title = item.value().attr("aria-label")
                .unwrap_or("DeviantArt Image")
                .to_string();

            // Get thumbnail image
            let img_el = item.select(&img_sel).next();
            let thumbnail_src = img_el
                .and_then(|el| el.value().attr("src"))
                .unwrap_or_default()
                .to_string();

            // Try to get full image from srcset
            let img_src = img_el
                .and_then(|el| el.value().attr("srcset"))
                .and_then(|srcset| {
                    let first = srcset.split(' ').next().unwrap_or_default();
                    if first.is_empty() { None } else {
                        // Remove /v1/... suffix for full resolution
                        if let Some(pos) = first.find("/v1") {
                            Some(first[..pos].to_string())
                        } else {
                            Some(first.to_string())
                        }
                    }
                })
                .unwrap_or_else(|| thumbnail_src.clone());

            if thumbnail_src.is_empty() {
                continue;
            }

            let mut result = SearchResult::new(
                title,
                href.to_string(),
                String::new(),
                "deviantart".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = "images".to_string();
            result.thumbnail = Some(thumbnail_src);
            result.image_src = Some(img_src);
            results.push(result);
        }

        Ok(results)
    }
}
