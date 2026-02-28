//! DigBT — torrent/file search engine via HTML scraping.
//!
//! Reference: <https://digbt.org>

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};

pub struct Digbt {
    client: Client,
}

impl Digbt {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Digbt {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "DigBT".to_string(),
            description: "DigBT torrent search engine".to_string(),
            categories: vec![SearchCategory::Files],
            base_url: "https://digbt.org".to_string(),
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let url = format!(
            "https://digbt.org/search/{}-time-{}",
            urlencoding::encode(&query.query),
            page
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let item_sel = Selector::parse("td.x-item")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let title_sel = Selector::parse("a[title]")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let files_sel = Selector::parse("div.files")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let tail_sel = Selector::parse("div.tail")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;

        let mut results = Vec::new();

        for item in document.select(&item_sel) {
            let link_el = match item.select(&title_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title = link_el.text().collect::<String>().trim().to_string();
            let href = link_el.value().attr("href").unwrap_or_default();

            if title.is_empty() || href.is_empty() {
                continue;
            }

            let full_url = format!("https://digbt.org{}", href);

            let content = item
                .select(&files_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let filesize = item
                .select(&tail_sel)
                .next()
                .map(|el| {
                    let text = el.text().collect::<String>();
                    let parts: Vec<&str> = text.split_whitespace().collect();
                    if parts.len() >= 5 {
                        format!("{} {}", parts[3], parts[4])
                    } else {
                        String::new()
                    }
                })
                .unwrap_or_default();

            let snippet = if filesize.is_empty() {
                content
            } else {
                format!("{} | Size: {}", content, filesize)
            };

            let mut result = SearchResult::new(title, full_url, snippet, "DigBT".to_string());
            result.category = Some(SearchCategory::Files);
            results.push(result);
        }

        Ok(results)
    }
}
