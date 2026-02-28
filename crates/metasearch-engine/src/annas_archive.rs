//! Anna's Archive — free shadow library metasearch for books and files.
//!
//! Reference: <https://annas-archive.org/>

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

pub struct AnnasArchive {
    client: Client,
}

impl AnnasArchive {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for AnnasArchive {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "Anna's Archive".to_string(),
            description: "Free non-profit online shadow library metasearch engine".to_string(),
            categories: vec![SearchCategory::Files],
            base_url: "https://annas-archive.org".to_string(),
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let url = format!(
            "https://annas-archive.org/search?q={}&page={}",
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

        // Anna's Archive wraps each result in a div inside js-aarecord-list-outer
        let item_sel = Selector::parse("main div.js-aarecord-list-outer > div")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let link_sel = Selector::parse("a[href^='/md5']")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let desc_sel = Selector::parse("div.relative")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;
        let img_sel = Selector::parse("img")
            .map_err(|e| MetasearchError::ParseError(format!("selector: {:?}", e)))?;

        let mut results = Vec::new();

        for item in document.select(&item_sel) {
            let link_el = match item.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title = link_el.text().collect::<String>().trim().to_string();
            let href = link_el.value().attr("href").unwrap_or_default();

            if title.is_empty() || href.is_empty() {
                continue;
            }

            let content = item
                .select(&desc_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let thumbnail = item
                .select(&img_sel)
                .next()
                .and_then(|el| el.value().attr("src"))
                .map(|s| s.to_string());

            let full_url = format!("https://annas-archive.org{}", href);

            let mut result =
                SearchResult::new(title, full_url, content, "Anna's Archive".to_string());
            result.category = Some(SearchCategory::Files);
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
