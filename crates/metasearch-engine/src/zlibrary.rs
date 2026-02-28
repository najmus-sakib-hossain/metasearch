//! Z-Library book search engine implementation.
//!
//! HTML scraping: <https://zlibrary-global.se/s/>
//! Website: https://zlibrary-global.se
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

const BASE_URL: &str = "https://zlibrary-global.se";

pub struct Zlibrary {
    metadata: EngineMetadata,
    client: Client,
}

impl Zlibrary {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "zlibrary".to_string(),
                display_name: "Z-Library".to_string(),
                homepage: BASE_URL.to_string(),
                categories: vec![SearchCategory::Files],
                enabled: true,
                timeout_ms: 6000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Zlibrary {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let encoded = urlencoding::encode(&query.query);
        let page = query.page.max(1);

        let url = format!("{}/s/{}?page={}", BASE_URL, encoded, page);

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

        let container_selector = Selector::parse("div#searchResultBox div.resItemBox")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let book_link_selector = Selector::parse("a[href^=\"/book/\"]")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let name_selector = Selector::parse("[itemprop=\"name\"]")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let author_selector = Selector::parse("div.authors a[itemprop=\"author\"]")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;

        let mut results = Vec::new();

        for (i, item) in document.select(&container_selector).enumerate() {
            // Extract book link
            let link_el = match item.select(&book_link_selector).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or_default();
            if href.is_empty() {
                continue;
            }

            let result_url = format!("{}{}", BASE_URL, href);

            // Extract title from itemprop="name"
            let title = match item.select(&name_selector).next() {
                Some(el) => el.text().collect::<String>().trim().to_string(),
                None => link_el.text().collect::<String>().trim().to_string(),
            };

            if title.is_empty() {
                continue;
            }

            // Extract authors
            let authors: Vec<String> = item
                .select(&author_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let content = if authors.is_empty() {
                String::new()
            } else {
                format!("by {}", authors.join(", "))
            };

            let mut r = SearchResult::new(&title, &result_url, &content, "zlibrary");
            r.engine_rank = i as u32;
            r.category = SearchCategory::Files.to_string();
            results.push(r);
        }

        info!(
            engine = "zlibrary",
            count = results.len(),
            "Search complete"
        );
        Ok(results)
    }
}
