//! Bing Videos engine — search videos via Bing Videos HTML scraping.
//! Translated from SearXNG `searx/engines/bing_videos.py`.

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

pub struct BingVideos {
    metadata: EngineMetadata,
    client: Client,
}

impl BingVideos {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "bing_videos".to_string(),
                display_name: "Bing Videos".to_string(),
                homepage: "https://www.bing.com".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 3000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for BingVideos {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page as u32;
        let first = (page - 1) * 35 + 1;

        let url = format!(
            "https://www.bing.com/videos/asyncv2?q={}&async=content&first={}&count=35",
            urlencoding::encode(&query.query),
            first,
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let video_sel = Selector::parse("div.dg_u div[id*='mc_vtvc_video']").unwrap();
        let meta_sel = Selector::parse("div.vrhdata").unwrap();
        let info_sel = Selector::parse("div.mc_vtvc_meta_block span").unwrap();
        let thumb_sel = Selector::parse("div.mc_vtvc_th img").unwrap();

        let mut results = Vec::new();

        for (i, video) in document.select(&video_sel).enumerate() {
            let meta_el = match video.select(&meta_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let meta_json = match meta_el.value().attr("vrhm") {
                Some(m) => m,
                None => continue,
            };

            let meta: serde_json::Value = match serde_json::from_str(meta_json) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let video_url = meta["murl"].as_str().unwrap_or_default();
            let title = meta["vt"].as_str().unwrap_or("Untitled");
            let duration = meta["du"].as_str().unwrap_or("");

            let info: Vec<String> = video.select(&info_sel)
                .map(|el| el.text().collect::<String>())
                .collect();
            let info_str = info.join(" - ");

            let thumbnail = video.select(&thumb_sel).next()
                .and_then(|el| el.value().attr("src"))
                .map(|s| s.to_string());

            let snippet = format!("{} — {}", duration, info_str.trim());

            let mut result = SearchResult::new(
                title.to_string(),
                video_url.to_string(),
                snippet,
                "bing_videos".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = "videos".to_string();
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
