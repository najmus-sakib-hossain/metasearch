//! Niconico video search engine implementation.
//!
//! HTML scraping: <https://www.nicovideo.jp/search/>
//! Website: https://www.nicovideo.jp
//! Features: Paging

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

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);

        let item_selector = Selector::parse("li[data-video-item]")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let thumb_link_selector = Selector::parse("a.itemThumbWrap")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let title_selector = Selector::parse("p.itemTitle a")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let desc_selector = Selector::parse("p.itemDescription")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let img_selector = Selector::parse("img.thumb")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;

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
