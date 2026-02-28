//! SolidTorrents — torrent search engine (HTML scraping)
//!
//! Searches solidtorrents.to for torrents.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};

use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};

const BASE_URL: &str = "https://solidtorrents.to";

pub struct SolidTorrents {
    client: Client,
}

impl SolidTorrents {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for SolidTorrents {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "solidtorrents".to_string(),
            display_name: "solidtorrents".to_string(),
            categories: vec![SearchCategory::Files],
            enabled: true,
            timeout_ms: 5000,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "{}/search?q={}&page={}",
            BASE_URL,
            urlencoding::encode(&query.query),
            query.page
        );

        let resp =
            self.client.get(&url).send().await.map_err(|e| {
                MetasearchError::Engine(format!("SolidTorrents request failed: {}", e))
            })?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::Engine(format!("SolidTorrents read failed: {}", e)))?;

        let document = Html::parse_document(&html_text);
        let result_sel = Selector::parse("li.search-result").unwrap();
        let title_sel = Selector::parse("h5.title").unwrap();
        let link_sel = Selector::parse("h5.title a").unwrap();
        let stats_sel = Selector::parse("div.stats div").unwrap();
        let category_sel = Selector::parse("a.category").unwrap();

        let mut results = Vec::new();

        for (i, element) in document.select(&result_sel).enumerate() {
            let title = element
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let href = element
                .select(&link_sel)
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or_default();

            let result_url = if href.starts_with("http") {
                href.to_string()
            } else {
                format!("{}{}", BASE_URL, href)
            };

            if title.is_empty() || result_url.is_empty() {
                continue;
            }

            let stats: Vec<String> = element
                .select(&stats_sel)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .collect();

            let category_text = element
                .select(&category_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let mut snippet_parts = Vec::new();
            if let Some(size) = stats.get(1) {
                snippet_parts.push(format!("Size: {}", size));
            }
            if let Some(seeders) = stats.get(3) {
                snippet_parts.push(format!("Seeds: {}", seeders));
            }
            if let Some(leechers) = stats.get(2) {
                snippet_parts.push(format!("Leeches: {}", leechers));
            }
            if !category_text.is_empty() {
                snippet_parts.push(category_text);
            }

            let snippet = snippet_parts.join(" | ");

            let mut result = SearchResult::new(&title, &result_url, &snippet, "solidtorrents");
            result.engine_rank = (i + 1) as u32;
            result.category = Some(SearchCategory::Files);
            results.push(result);
        }

        Ok(results)
    }
}
