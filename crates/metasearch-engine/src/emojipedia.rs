//! Emojipedia search engine.
//!
//! Emojipedia is an emoji reference website documenting the meaning
//! and common usage of emoji characters in the Unicode Standard.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

use metasearch_core::engine::{EngineMetadata, SearchEngine};
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;
use metasearch_core::error::MetasearchError;
use metasearch_core::category::SearchCategory;

const BASE_URL: &str = "https://emojipedia.org";

pub struct Emojipedia {
    client: Client,
}

impl Emojipedia {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Emojipedia {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "Emojipedia".to_string(),
            base_url: BASE_URL.to_string(),
            categories: vec![SearchCategory::General],
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let mut url = Url::parse(&format!("{}/search", BASE_URL))
            .map_err(|e| MetasearchError::Request(e.to_string()))?;
        url.query_pairs_mut().append_pair("q", &query.query);

        let resp = self
            .client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| MetasearchError::Request(e.to_string()))?
            .text()
            .await
            .map_err(|e| MetasearchError::Request(e.to_string()))?;

        let document = Html::parse_document(&resp);

        // Emojipedia uses div class starting with "EmojisList" containing <a> links
        let container_sel = Selector::parse("div[class^='EmojisList'] a").unwrap();

        let mut results = Vec::new();

        for element in document.select(&container_sel) {
            let href = element.value().attr("href").unwrap_or_default();
            let title: String = element.text().collect::<Vec<_>>().join(" ").trim().to_string();

            if title.is_empty() || href.is_empty() {
                continue;
            }

            let emoji_url = format!("{}{}", BASE_URL, href);

            results.push(SearchResult {
                title,
                url: emoji_url,
                content: String::new(),
                engine: "Emojipedia".to_string(),
                score: 1.0,
            });
        }

        Ok(results)
    }
}
