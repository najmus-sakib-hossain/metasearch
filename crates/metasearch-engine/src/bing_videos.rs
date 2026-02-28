//! Bing Videos engine — search videos via Bing Videos HTML scraping.
//! Translated from SearXNG `searx/engines/bing_videos.py`.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};

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
                homepage: "https://www.bing.com/videos".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 5000,
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

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let first = (page - 1) * 35 + 1;

        let url = format!(
            "https://www.bing.com/videos/asyncv2?q={}&async=content&first={}&count=35",
            urlencoding::encode(&query.query),
            first,
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        // Each video card is a div with id starting with "mc_vtvc_video_"
        let video_sel = Selector::parse("div[id^='mc_vtvc_video_']").unwrap();
        // Title is in a child element whose "title" attribute holds the full title text
        let title_sel = Selector::parse("div.mc_vtvc_title").unwrap();
        // Meta spans contain things like view count, channel name, upload date
        let info_sel = Selector::parse("div.mc_vtvc_meta_block span").unwrap();

        let mut results = Vec::new();

        for (i, video) in document.select(&video_sel).enumerate() {
            // The "mmeta" attribute on the card div contains JSON with murl (video URL)
            // and turl (thumbnail URL).
            let mmeta_raw = match video.value().attr("mmeta") {
                Some(m) => m,
                None => continue,
            };

            // Bing HTML-encodes the JSON (e.g. &quot; instead of "), decode it first.
            let mmeta_decoded = mmeta_raw
                .replace("&quot;", "\"")
                .replace("&amp;", "&")
                .replace("&#39;", "'")
                .replace("&lt;", "<")
                .replace("&gt;", ">");

            let meta: serde_json::Value = match serde_json::from_str(&mmeta_decoded) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let video_url = match meta["murl"].as_str() {
                Some(u) if !u.is_empty() => u.to_string(),
                _ => continue,
            };

            // Title is the "title" attribute of the mc_vtvc_title element
            let title = video
                .select(&title_sel)
                .next()
                .and_then(|el| el.value().attr("title"))
                .unwrap_or("Untitled")
                .to_string();

            let thumbnail = meta["turl"].as_str().map(|s| s.to_string());

            let info: Vec<String> = video
                .select(&info_sel)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let snippet = info.join(" · ");

            let mut result = SearchResult::new(
                title,
                video_url,
                snippet,
                "bing_videos".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::Videos.to_string();
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
