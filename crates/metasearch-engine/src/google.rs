//! Google search engine implementation.
//!
//! Scrapes Google's web search results page. Inspired by SearXNG's `google.py`.
//! Uses the `&num=` parameter for pagination and parses organic results.

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

pub struct Google {
    metadata: EngineMetadata,
    client: Client,
}

impl Google {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "google".to_string(),
                display_name: "Google".to_string(),
                homepage: "https://www.google.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.5,
            },
            client,
        }
    }

    /// Map safe_search level to Google's `safe` parameter value.
    fn safe_search_param(level: u8) -> &'static str {
        match level {
            2 => "active",
            1 => "medium",
            _ => "off",
        }
    }
}

#[async_trait]
impl SearchEngine for Google {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let start = (query.page.max(1) - 1) * 10;
        let lang = query.language.as_deref().unwrap_or("en");
        let safe = Self::safe_search_param(query.safe_search);

        let url = format!(
            "https://www.google.com/search?q={}&start={}&num=10&hl={}&safe={}&gbv=1&ie=utf8&oe=utf8",
            urlencoding::encode(&query.query),
            start,
            lang,
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
            .header("DNT", "1")
            .header("Referer", "https://www.google.com/")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let mut results = Vec::new();

        // Strategy 1: Modern Google layout — <div class="g"> containers
        let g_selector = Selector::parse("div.g").unwrap();
        let title_sel_a = Selector::parse("h3").unwrap();
        let link_sel = Selector::parse("a[href]").unwrap();
        let snippet_sel = Selector::parse("div.VwiC3b, span.aCOpRe, div[data-sncf], div[style*='-webkit-line-clamp']").unwrap();

        for (i, element) in document.select(&g_selector).enumerate() {
            // Get the link — first <a> with href
            let link_el = match element.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };
            let href = link_el.value().attr("href").unwrap_or_default();
            // Skip Google's internal links
            if href.is_empty() || href.starts_with('/') || href.starts_with('#') {
                continue;
            }
            // Handle Google redirect URLs
            let result_url = if href.contains("/url?q=") {
                href.split("/url?q=")
                    .nth(1)
                    .and_then(|s| s.split('&').next())
                    .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
                    .unwrap_or_else(|| href.to_string())
            } else {
                href.to_string()
            };

            if !result_url.starts_with("http") {
                continue;
            }

            // Get title from <h3>
            let title: String = element
                .select(&title_sel_a)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if title.is_empty() {
                continue;
            }

            // Get snippet
            let snippet: String = element
                .select(&snippet_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let mut r = SearchResult::new(&title, &result_url, &snippet, "google");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        // Strategy 2: Fallback — try alternative selectors if no results from Strategy 1
        if results.is_empty() {
            let alt_result_sel =
                Selector::parse("div.tF2Cxc, div.yuRUbf, div[class*='result']").unwrap();
            let alt_title_sel = Selector::parse("h3, a[data-ved] > div").unwrap();
            let alt_link_sel = Selector::parse("a").unwrap();
            let alt_snippet_sel =
                Selector::parse("div.IsZvec, span.st, div.s, div[data-content-feature]").unwrap();

            for (i, element) in document.select(&alt_result_sel).enumerate() {
                let link_el = match element.select(&alt_link_sel).next() {
                    Some(el) => el,
                    None => continue,
                };
                let href = link_el.value().attr("href").unwrap_or_default();
                if href.is_empty() || href.starts_with('/') || href.starts_with('#') {
                    continue;
                }
                let result_url = if href.contains("/url?q=") {
                    href.split("/url?q=")
                        .nth(1)
                        .and_then(|s| s.split('&').next())
                        .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
                        .unwrap_or_else(|| href.to_string())
                } else {
                    href.to_string()
                };
                if !result_url.starts_with("http") {
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

                let mut r = SearchResult::new(&title, &result_url, &snippet, "google");
                r.engine_rank = (i + 1) as u32;
                r.category = SearchCategory::General.to_string();
                results.push(r);

                if results.len() >= 10 {
                    break;
                }
            }
        }

        info!(engine = "google", count = results.len(), "Search complete");
        Ok(results)
    }
}
