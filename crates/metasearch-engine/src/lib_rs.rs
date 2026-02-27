//! lib.rs engine — search Rust crates on lib.rs.
//! Translated from SearXNG `searx/engines/lib_rs.py`.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct LibRs {
    client: Client,
}

impl LibRs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for LibRs {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "lib_rs".to_string(),
            display_name: "lib.rs".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 0.7,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://lib.rs/search?q={}",
            urlencoding::encode(&query.query),
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let result_sel = Selector::parse("main div ol li a").unwrap();
        let title_sel = Selector::parse("div.h h4").unwrap();
        let content_sel = Selector::parse("div.h p").unwrap();
        let version_sel = Selector::parse("div.meta span.version, div.meta span.v").unwrap();
        let downloads_sel = Selector::parse("div.meta span.downloads").unwrap();

        let mut results = Vec::new();

        for (i, item) in document.select(&result_sel).enumerate() {
            let href = item.value().attr("href").unwrap_or_default();
            let link = format!("https://lib.rs{}", href);

            let title = item.select(&title_sel).next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let content = item.select(&content_sel).next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let version = item.select(&version_sel).next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let downloads = item.select(&downloads_sel).next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let snippet = if version.is_empty() && downloads.is_empty() {
                content.clone()
            } else {
                format!("{} | v{} | {}", content, version.trim(), downloads.trim())
            };

            let mut result = SearchResult::new(
                title,
                link,
                snippet,
                "lib_rs".to_string(),
            );
            result.engine_rank = Some(i + 1);
            result.category = Some(SearchCategory::IT);
            results.push(result);
        }

        Ok(results)
    }
}
