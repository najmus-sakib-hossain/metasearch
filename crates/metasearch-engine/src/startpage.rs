//! Startpage search engine implementation.
//!
//! HTML scraping via POST: <https://www.startpage.com/sp/search>
//! Website: https://www.startpage.com
//! Features: Web search, pagination, time range filtering

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

pub struct Startpage {
    metadata: EngineMetadata,
    client: Client,
}

impl Startpage {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "startpage".to_string(),
                display_name: "Startpage".to_string(),
                homepage: "https://www.startpage.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }

    /// Map time_range string to Startpage's `with_date` parameter.
    fn map_time_range(time_range: Option<&str>) -> Option<&'static str> {
        match time_range {
            Some("day") => Some("d"),
            Some("week") => Some("w"),
            Some("month") => Some("m"),
            Some("year") => Some("y"),
            _ => None,
        }
    }
}

#[async_trait]
impl SearchEngine for Startpage {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page.max(1);

        let mut form_data = vec![
            ("query".to_string(), query.query.clone()),
            ("cat".to_string(), "web".to_string()),
            ("page".to_string(), page.to_string()),
        ];

        if let Some(date_param) = Self::map_time_range(query.time_range.as_deref()) {
            form_data.push(("with_date".to_string(), date_param.to_string()));
        }

        let resp = self
            .client
            .post("https://www.startpage.com/sp/search")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .header("Accept", "text/html,application/xhtml+xml")
            .header("Accept-Language", "en-US,en;q=0.9")
            .form(&form_data)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);

        let mut results = Vec::new();

        // Primary selector: Startpage result containers
        let result_sel = Selector::parse(".w-gl__result")
            .unwrap_or_else(|_| Selector::parse("div.result").expect("selector should parse"));
        let title_sel = Selector::parse(".w-gl__result-title, h3 a, h3")
            .unwrap_or_else(|_| Selector::parse("h3").expect("selector should parse"));
        let link_sel =
            Selector::parse("a[href]").expect("link selector should parse");
        let desc_sel = Selector::parse(".w-gl__description, p.w-gl__description, p")
            .unwrap_or_else(|_| Selector::parse("p").expect("selector should parse"));

        for (i, container) in document.select(&result_sel).enumerate() {
            // Extract title — try title selector first, then any link
            let title_text = container
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if title_text.is_empty() {
                continue;
            }

            // Extract URL — look for href in links
            let href = container
                .select(&link_sel)
                .find_map(|a| {
                    // Prefer data-url attribute if available
                    a.value()
                        .attr("data-url")
                        .or_else(|| a.value().attr("href"))
                        .filter(|h| h.starts_with("http") && !h.contains("startpage.com"))
                        .map(|h| h.to_string())
                })
                .unwrap_or_default();

            if href.is_empty() {
                continue;
            }

            // Extract description
            let content: String = container
                .select(&desc_sel)
                .next()
                .map(|el| {
                    el.text()
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();

            let mut r = SearchResult::new(&title_text, &href, &content, "startpage");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);
        }

        // Fallback: broader approach if primary selectors didn't match
        if results.is_empty() {
            let broad_sel = Selector::parse("[class*='gl__result'], [class*='result']")
                .unwrap_or_else(|_| Selector::parse("div").expect("selector should parse"));

            for (i, container) in document.select(&broad_sel).enumerate() {
                let link = match container.select(&link_sel).next() {
                    Some(l) => l,
                    None => continue,
                };

                let href = link
                    .value()
                    .attr("data-url")
                    .or_else(|| link.value().attr("href"))
                    .unwrap_or_default();

                if href.is_empty()
                    || !href.starts_with("http")
                    || href.contains("startpage.com")
                {
                    continue;
                }

                let title: String = link.text().collect::<String>().trim().to_string();
                if title.is_empty() {
                    continue;
                }

                let content: String = container
                    .select(&desc_sel)
                    .next()
                    .map(|el| {
                        el.text()
                            .collect::<String>()
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .unwrap_or_default();

                let mut r = SearchResult::new(&title, href, &content, "startpage");
                r.engine_rank = (i + 1) as u32;
                r.category = SearchCategory::General.to_string();
                results.push(r);

                if results.len() >= 10 {
                    break;
                }
            }
        }

        Ok(results)
    }
}
