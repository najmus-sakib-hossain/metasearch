//! Naver — Korean search engine.
//!
//! Scrapes HTML search results from search.naver.com.
//!
//! Reference: <https://search.naver.com>

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};

use metasearch_core::category::SearchCategory;
use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::error::MetasearchError;
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;

const BASE_URL: &str = "https://search.naver.com";
const RESULTS_PER_PAGE: u32 = 15;

pub struct Naver {
    metadata: EngineMetadata,
    client: Client,
}

impl Naver {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "naver".to_string(),
                display_name: "Naver".to_string(),
                homepage: BASE_URL.to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Naver {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let start = (query.page.saturating_sub(1)) * RESULTS_PER_PAGE + 1;

        let url = format!(
            "{}/search.naver?query={}&where=web&start={}",
            BASE_URL,
            urlencoding::encode(&query.query),
            start,
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

        // Naver web results appear in list items within the total results list
        let item_sel = Selector::parse("ul.lst_total li.bx")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let title_sel = Selector::parse("a.link_tit")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let desc_sel = Selector::parse("div.total_dsc_wrap a.api_txt_lines")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (i, item) in document.select(&item_sel).enumerate() {
            let title_el = match item.select(&title_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = title_el.value().attr("href").unwrap_or_default();
            if href.is_empty() {
                continue;
            }

            let title: String = title_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let content: String = item
                .select(&desc_sel)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            if title.is_empty() {
                continue;
            }

            let mut result = SearchResult::new(title, href.to_string(), content, "naver");
            result.engine_rank = (i + 1) as u32;
            results.push(result);
        }

        Ok(results)
    }
}
