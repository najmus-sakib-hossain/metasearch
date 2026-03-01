//! Niconico video search engine implementation.
//!
//! HTML scraping: <https://www.nicovideo.jp/search/>
//! Website: https://www.nicovideo.jp
//! Features: Paging

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::Result,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::info;

pub struct Niconico {
    metadata: EngineMetadata,
    client: Client,
}

impl Niconico {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "niconico".to_string(),
                display_name: "Niconico".to_string(),
                homepage: "https://www.nicovideo.jp".to_string(),
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
impl SearchEngine for Niconico {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let encoded = urlencoding::encode(&query.query);
        let page = query.page.max(1);

        let url = format!(
            "https://www.nicovideo.jp/search/{}?page={}",
            encoded, page
        );

        let resp = match self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(6))
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(_) => return Ok(Vec::new()),
        };

        if !resp.status().is_success() {
            return Ok(Vec::new());
        }

        let body = match resp.text().await {
            Ok(t) => t,
            Err(_) => return Ok(Vec::new()),
        };

        let document = Html::parse_document(&body);

        let item_selector = match Selector::parse("li[data-video-item]") {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        let thumb_link_selector = match Selector::parse("a.itemThumbWrap") {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        let title_selector = match Selector::parse("p.itemTitle a") {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        let desc_selector = match Selector::parse("p.itemDescription") {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        let img_selector = match Selector::parse("img.thumb") {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };

        let mut results = Vec::new();

        for (i, item) in document.select(&item_selector).enumerate() {
            // Extract video ID from thumbnail link
            let video_id = match item.select(&thumb_link_selector).next() {
                Some(el) => {
                    let href = el.value().attr("href").unwrap_or_default();
                    // href may be like /watch/sm12345 or just sm12345
                    let id = href
                        .rsplit('/')
                        .next()
                        .unwrap_or(href)
                        .trim()
                        .to_string();
                    if id.is_empty() {
                        continue;
                    }
                    id
                }
                None => continue,
            };

            let result_url = format!("https://www.nicovideo.jp/watch/{}", video_id);

            let title = match item.select(&title_selector).next() {
                Some(el) => el.text().collect::<String>().trim().to_string(),
                None => continue,
            };

            if title.is_empty() {
                continue;
            }

            let content = item
                .select(&desc_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let thumbnail = item
                .select(&img_selector)
                .next()
                .and_then(|el| el.value().attr("src").map(|s| s.to_string()));

            let mut r = SearchResult::new(&title, &result_url, &content, "niconico");
            r.engine_rank = i as u32;
            r.category = SearchCategory::Videos.to_string();
            r.thumbnail = thumbnail;
            results.push(r);
        }

        info!(
            engine = "niconico",
            count = results.len(),
            "Search complete"
        );
        Ok(results)
    }
}
