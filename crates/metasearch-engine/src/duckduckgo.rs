//! DuckDuckGo search engine implementation.
//! Uses the HTML-lite POST endpoint at html.duckduckgo.com.

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

pub struct DuckDuckGo {
    metadata: EngineMetadata,
    client: Client,
}

impl DuckDuckGo {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "duckduckgo".to_string(),
                display_name: "DuckDuckGo".to_string(),
                homepage: "https://duckduckgo.com".to_string(),
                categories: vec![
                    SearchCategory::General,
                    SearchCategory::Images,
                    SearchCategory::News,
                ],
                enabled: true,
                timeout_ms: 6000,
                weight: 1.2,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for DuckDuckGo {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Use DuckDuckGo's HTML lite POST endpoint which doesn't require JS.
        // Pagination: s=0 (page 1), s=30 (page 2), s=60 (page 3), ...
        let offset = (query.page.saturating_sub(1)) * 30;
        let body = if offset == 0 {
            format!("q={}", urlencoding::encode(&query.query))
        } else {
            format!(
                "q={}&s={}&dc={}&v=l&o=json&api=/d.js",
                urlencoding::encode(&query.query),
                offset,
                offset + 1
            )
        };

        let resp = self
            .client
            .post("https://html.duckduckgo.com/html/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0",
            )
            .body(body)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text: String = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);

        // Each organic result is a <div class="result results_links results_links_deep web-result ">
        let result_sel =
            Selector::parse("div.result.results_links_deep.web-result > div.links_main")
                .unwrap_or_else(|_| Selector::parse("div.links_main").unwrap());
        let title_sel = Selector::parse("h2.result__title a.result__a").unwrap();
        let snippet_sel = Selector::parse("a.result__snippet").unwrap();
        let url_sel = Selector::parse("span.result__url").unwrap();

        let mut results = Vec::new();

        for (i, item) in document.select(&result_sel).enumerate() {
            let title_el = match item.select(&title_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title: String = title_el.text().collect();
            let title = title.trim().to_string();
            if title.is_empty() {
                continue;
            }

            // The href on DDG is wrapped in a redirect — get the actual URL from the span
            let displayed_url: String = item
                .select(&url_sel)
                .next()
                .map(|e| e.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            // Try to build a canonical URL from the displayed domain
            let canonical_url = if displayed_url.starts_with("http") {
                displayed_url.clone()
            } else if !displayed_url.is_empty() {
                format!("https://{}", displayed_url)
            } else {
                // Fall back to the href on the result title
                title_el
                    .value()
                    .attr("href")
                    .unwrap_or_default()
                    .to_string()
            };

            if canonical_url.is_empty() {
                continue;
            }

            let snippet: String = item
                .select(&snippet_sel)
                .next()
                .map(|e| {
                    e.text()
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();

            let mut r = SearchResult::new(&title, &canonical_url, &snippet, "duckduckgo");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        Ok(results)
    }
}
