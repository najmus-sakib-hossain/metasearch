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
use smallvec::smallvec;

pub struct Google {
    metadata: EngineMetadata,
    client: Client,
}

impl Google {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "google".to_string().into(),
                display_name: "Google".to_string().into(),
                homepage: "https://www.google.com".to_string().into(),
                categories: smallvec![SearchCategory::General],
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

        // Generate arc_id similar to SearXNG (simplified version)
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let arc_id = format!("arc_id:srp_{}_{:02}", timestamp / 3600, start);
        let async_param = format!("{},use_ac:true,_fmt:prog", arc_id);

        let url = format!(
            "https://www.google.com/search?q={}&start={}&hl={}&lr=lang_{}&ie=utf8&oe=utf8&filter=0&safe={}&asearch=arc&async={}",
            urlencoding::encode(&query.query),
            start,
            lang,
            lang,
            safe,
            urlencoding::encode(&async_param),
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .header("Accept-Language", format!("{},en-US;q=0.9,en;q=0.8", lang))
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Referer", "https://www.google.com/")
            .header("Cookie", "CONSENT=YES+")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let mut results = Vec::new();

        // SearXNG uses: .//div[contains(@class, "MjjYud")]
        // Try multiple selectors for robustness
        let selectors_to_try = vec![
            "div.MjjYud",
            "div.g",
            "div.tF2Cxc",
            "div[data-sokoban-container]",
        ];

        for selector_str in selectors_to_try {
            if !results.is_empty() {
                break;
            }

            let result_selector = match Selector::parse(selector_str) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let title_sel = Selector::parse("h3, div[role='link']").unwrap();
            let link_sel = Selector::parse("a[href]").unwrap();
            let snippet_sel = Selector::parse("div[data-sncf='1'], div.VwiC3b, span.aCOpRe, div.s, div.IsZvec").unwrap();

            for (i, element) in document.select(&result_selector).enumerate() {
                // Get the link
                let link_el = match element.select(&link_sel).next() {
                    Some(el) => el,
                    None => continue,
                };
                
                let href = link_el.value().attr("href").unwrap_or_default();
                if href.is_empty() || href.starts_with('/') || href.starts_with('#') {
                    continue;
                }

                // Handle Google redirect URLs: /url?q=...
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

                // Get title
                let title: String = element
                    .select(&title_sel)
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

                if results.len() >= 10 {
                    break;
                }
            }
        }

        info!(engine = "google", count = results.len(), "Search complete");
        Ok(results)
    }
}
