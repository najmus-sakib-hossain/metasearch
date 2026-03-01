//! Brave Search engine implementation.
//!
//! Scrapes Brave Search HTML results. Inspired by SearXNG's `brave.py`.
//! Supports general web search with optional API key for JSON API.

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
use std::borrow::Cow;
use tracing::info;
use smallvec::smallvec;

#[allow(dead_code)]
pub struct Brave {
    metadata: EngineMetadata,
    client: Client,
    api_key: Option<String>,
}

impl Brave {
    pub fn new(client: Client, api_key: Option<String>) -> Self {
        Self {
            metadata: EngineMetadata {
                name: Cow::Borrowed("brave"),
                display_name: Cow::Borrowed("Brave Search"),
                homepage: Cow::Borrowed("https://search.brave.com"),
                categories: smallvec![SearchCategory::General, SearchCategory::News],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.3,
            },
            client,
            api_key,
        }
    }

    /// Map safe_search level to Brave's safesearch parameter.
    fn safe_search_param(level: u8) -> &'static str {
        match level {
            2 => "strict",
            1 => "moderate",
            _ => "off",
        }
    }
}

#[async_trait]
impl SearchEngine for Brave {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let offset = (query.page.max(1) - 1) * 10;
        let lang = query.language.as_deref().unwrap_or("en");
        let safe = Self::safe_search_param(query.safe_search);

        let url = format!(
            "https://search.brave.com/search?q={}&offset={}&spellcheck=1&source=web&safesearch={}",
            urlencoding::encode(&query.query),
            offset,
            safe,
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header("Accept-Language", format!("{},en;q=0.5", lang))
            .header("Accept-Encoding", "gzip, deflate")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let mut results = Vec::new();

        // Brave organic results: <div class="snippet">
        let result_sel = Selector::parse("div.snippet[data-type='web']").unwrap();
        let title_sel = Selector::parse("a.result-header span.snippet-title").unwrap();
        let link_sel = Selector::parse("a.result-header").unwrap();
        let snippet_sel = Selector::parse("p.snippet-description").unwrap();

        for (i, element) in document.select(&result_sel).enumerate() {
            let link_el = match element.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };
            let result_url = link_el
                .value()
                .attr("href")
                .unwrap_or_default()
                .to_string();
            if result_url.is_empty() || !result_url.starts_with("http") {
                continue;
            }

            let title: String = element
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if title.is_empty() {
                continue;
            }

            let snippet: String = element
                .select(&snippet_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let mut r = SearchResult::new(&title, &result_url, &snippet, "brave");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        // Fallback: try alternative Brave HTML structure
        if results.is_empty() {
            let alt_sel = Selector::parse("div.snippet, div[data-type='web']").unwrap();
            let alt_title_sel = Selector::parse("span.snippet-title, div.title").unwrap();
            let alt_link_sel = Selector::parse("a[href^='http']").unwrap();
            let alt_snippet_sel =
                Selector::parse("p.snippet-description, div.snippet-description, p.desc")
                    .unwrap();

            for (i, element) in document.select(&alt_sel).enumerate() {
                let link_el = match element.select(&alt_link_sel).next() {
                    Some(el) => el,
                    None => continue,
                };
                let result_url = link_el
                    .value()
                    .attr("href")
                    .unwrap_or_default()
                    .to_string();
                if result_url.is_empty()
                    || !result_url.starts_with("http")
                    || result_url.contains("brave.com")
                {
                    continue;
                }

                let title: String = element
                    .select(&alt_title_sel)
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .unwrap_or_default();
                if title.is_empty() {
                    continue;
                }

                let snippet: String = element
                    .select(&alt_snippet_sel)
                    .next()
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .unwrap_or_default();

                let mut r = SearchResult::new(&title, &result_url, &snippet, "brave");
                r.engine_rank = (i + 1) as u32;
                r.category = SearchCategory::General.to_string();
                results.push(r);

                if results.len() >= 10 {
                    break;
                }
            }
        }

        info!(engine = "brave", count = results.len(), "Search complete");
        Ok(results)
    }
}
