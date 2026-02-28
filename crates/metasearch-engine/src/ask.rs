//! Ask.com search engine implementation.
//!
//! Translated from SearXNG's `ask.py` (75 lines, HTML + JS JSON).
//! Ask.com is a general web search engine.
//! Website: https://www.ask.com/
//! Features: Paging (max 5 pages)

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

pub struct Ask {
    metadata: EngineMetadata,
    client: Client,
}

impl Ask {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "ask".to_string(),
                display_name: "Ask.com".to_string(),
                homepage: "https://www.ask.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 4000,
                weight: 0.8,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Ask {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Ask.com has at max 5 pages
        let page = query.page.clamp(1, 5);
        let encoded = urlencoding::encode(&query.query);

        let url = format!("https://www.ask.com/web?q={}&page={}", encoded, page);

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

        // Ask.com results are in <div class="PartialSearchResults-item">
        let result_selector = Selector::parse(".PartialSearchResults-item, .result").unwrap();
        let title_selector =
            Selector::parse(".PartialSearchResults-item-title a, .result-title a, a.result-link")
                .unwrap();
        let snippet_selector = Selector::parse(
            ".PartialSearchResults-item-abstract, .result-abstract, p.result-url + p",
        )
        .unwrap();

        let mut results = Vec::new();

        for (i, element) in document.select(&result_selector).enumerate() {
            let title_el = match element.select(&title_selector).next() {
                Some(el) => el,
                None => continue,
            };

            let title: String = title_el.text().collect();
            let mut result_url = match title_el.value().attr("href") {
                Some(href) => href.to_string(),
                None => continue,
            };

            // Ask.com often wraps URLs with a redirect — strip &ueid suffix
            if let Some(idx) = result_url.find("&ueid") {
                result_url.truncate(idx);
            }

            let snippet: String = element
                .select(&snippet_selector)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            if !title.is_empty() && !result_url.is_empty() {
                let mut r = SearchResult::new(&title, &result_url, &snippet, "ask");
                r.engine_rank = i as u32;
                r.category = SearchCategory::General.to_string();
                results.push(r);
            }
        }

        info!(engine = "ask", count = results.len(), "Search complete");
        Ok(results)
    }
}
