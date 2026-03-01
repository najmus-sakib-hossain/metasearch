//! Bing search engine implementation.
//!
//! Translated from SearXNG's `bing.py` (279 lines, HTML scraping).
//! Bing is Microsoft's web search engine supporting general, images, news, and videos.
//! Website: https://www.bing.com
//! Features: Paging, SafeSearch, Time Range

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
use smallvec::smallvec;

pub struct Bing {
    metadata: EngineMetadata,
    client: Client,
}

impl Bing {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "bing".to_string().into(),
                display_name: "Bing".to_string().into(),
                homepage: "https://www.bing.com".to_string().into(),
                categories: smallvec![
                    SearchCategory::General,
                    SearchCategory::News,
                    SearchCategory::Images,
                ],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.4,
            },
            client,
        }
    }

    /// Map time_range to Bing's `filters` query parameter.
    fn time_range_param(time_range: Option<&str>) -> Option<String> {
        match time_range {
            Some("day") => Some("ex1%3a%22ez1%22".to_string()),
            Some("week") => Some("ex1%3a%22ez2%22".to_string()),
            Some("month") => Some("ex1%3a%22ez3%22".to_string()),
            Some("year") => Some("ex1%3a%22ez5_19868_20133%22".to_string()),
            _ => None,
        }
    }

    /// Map safe_search level to Bing's cookie.
    fn safesearch_cookie(level: u8) -> &'static str {
        match level {
            2 => "STRICT",
            1 => "DEMOTE",
            _ => "OFF",
        }
    }
}

#[async_trait]
impl SearchEngine for Bing {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let offset = ((query.page.max(1) - 1) * 10) as u64;
        let encoded_query = urlencoding::encode(&query.query);

        let mut url = format!(
            "https://www.bing.com/search?q={}&first={}&count=10",
            encoded_query,
            offset + 1
        );

        if let Some(filter) = Self::time_range_param(query.time_range.as_deref()) {
            url.push_str(&format!("&filters={}", filter));
        }

        let lang = query.language.as_deref().unwrap_or("en");
        let safesearch = Self::safesearch_cookie(query.safe_search);

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept-Language", format!("{},en;q=0.9", lang))
            .header("Cookie", format!("SRCHHPGUSR=ADLT={}", safesearch))
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);

        // Bing organic results are in <li class="b_algo">
        let result_selector = Selector::parse("li.b_algo").unwrap();
        let title_selector = Selector::parse("h2 a").unwrap();
        let snippet_selector = Selector::parse(".b_caption p, .b_lineclamp2").unwrap();

        let mut results = Vec::new();

        for (i, element) in document.select(&result_selector).enumerate() {
            let title_el = match element.select(&title_selector).next() {
                Some(el) => el,
                None => continue,
            };

            let title: String = title_el.text().collect();
            let result_url = match title_el.value().attr("href") {
                Some(href) => href.to_string(),
                None => continue,
            };

            let snippet: String = element
                .select(&snippet_selector)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let mut r = SearchResult::new(&title, &result_url, &snippet, "bing");
            r.engine_rank = i as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        info!(engine = "bing", count = results.len(), "Search complete");
        Ok(results)
    }
}
