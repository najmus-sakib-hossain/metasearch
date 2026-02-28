//! Arch Linux Wiki engine — search the Arch Linux Wiki via HTML scraping.
//! Translated from SearXNG `searx/engines/archlinux.py`.
//!
//! Note: The original SearXNG engine supports i18n with multiple wiki
//! subdomains. This Rust version searches the English wiki only.

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

pub struct ArchLinux {
    metadata: EngineMetadata,
    client: Client,
}

impl ArchLinux {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "archlinux".to_string(),
                display_name: "Arch Linux Wiki".to_string(),
                homepage: "https://wiki.archlinux.org".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 5000,
                weight: 0.8,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for ArchLinux {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let offset = (page - 1) * 20;

        let url = format!(
            "https://wiki.archlinux.org/index.php?search={}&title=Special%3ASearch&limit=20&offset={}&profile=default",
            urlencoding::encode(&query.query),
            offset,
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; metasearch/1.0)")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let result_sel = Selector::parse("ul.mw-search-results li").unwrap();
        let heading_sel = Selector::parse("div.mw-search-result-heading a").unwrap();
        let content_sel = Selector::parse("div.searchresult").unwrap();

        let mut results = Vec::new();

        for (i, item) in document.select(&result_sel).enumerate() {
            let link = match item.select(&heading_sel).next() {
                Some(l) => l,
                None => continue,
            };

            let href = link.value().attr("href").unwrap_or_default();
            let title = link.text().collect::<String>();

            if title.is_empty() {
                continue;
            }

            let item_url = format!("https://wiki.archlinux.org{}", href);

            let content = item
                .select(&content_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let mut result = SearchResult::new(
                title.trim().to_string(),
                item_url,
                content.trim().to_string(),
                "archlinux".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::IT.to_string();
            results.push(result);
        }

        Ok(results)
    }
}
