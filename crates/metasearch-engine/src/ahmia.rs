//! Ahmia — search engine for Tor hidden services.
//!
//! Scrapes HTML search results from ahmia.fi.
//!
//! Reference: <https://ahmia.fi>

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

use metasearch_core::category::SearchCategory;
use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::error::MetasearchError;
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;

const BASE_URL: &str = "https://ahmia.fi";

pub struct Ahmia {
    metadata: EngineMetadata,
    client: Client,
}

impl Ahmia {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "ahmia".to_string(),
                display_name: "Ahmia".to_string(),
                homepage: BASE_URL.to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 10000,
                weight: 0.5,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Ahmia {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "{}/search/?q={}",
            BASE_URL,
            urlencoding::encode(&query.query),
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let result_sel = Selector::parse("li.result")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel = Selector::parse("h4 a")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let title_sel = Selector::parse("h4")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let desc_sel = Selector::parse("p")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (i, item) in document.select(&result_sel).enumerate() {
            let link_el = match item.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or_default();

            // Ahmia redirect URLs contain the actual destination as a query param
            let result_url = if href.contains("redirect") {
                let full = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", BASE_URL, href)
                };
                Url::parse(&full)
                    .ok()
                    .and_then(|u| {
                        u.query_pairs()
                            .find(|(k, _)| k == "search_url")
                            .map(|(_, v)| v.to_string())
                    })
                    .unwrap_or(full)
            } else if href.starts_with("http") {
                href.to_string()
            } else {
                format!("{}{}", BASE_URL, href)
            };

            let title: String = item
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            let content: String = item
                .select(&desc_sel)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            if title.is_empty() && result_url.is_empty() {
                continue;
            }

            let mut result = SearchResult::new(title, result_url, content, "ahmia");
            result.engine_rank = (i + 1) as u32;
            results.push(result);
        }

        Ok(results)
    }
}
