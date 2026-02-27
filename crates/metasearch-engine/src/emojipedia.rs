//! Emojipedia engine — search emoji via HTML scraping.
//! Translated from SearXNG `searx/engines/emojipedia.py`.

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
            name: "emojipedia".to_string(),
            display_name: "Emojipedia".to_string(),
            categories: vec![SearchCategory::General],
            enabled: true,
            weight: 0.5,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://emojipedia.org/search?q={}",
            urlencoding::encode(&query.query),
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let list_sel = Selector::parse(r#"div[class^="EmojisList"] a"#).unwrap();

        let mut results = Vec::new();

        for (i, el) in document.select(&list_sel).enumerate() {
            let href = el.value().attr("href").unwrap_or_default();
            let result_url = if href.starts_with("http") {
                href.to_string()
            } else {
                format!("https://emojipedia.org{}", href)
            };

            let title: String = el.text().collect();
            if title.trim().is_empty() {
                continue;
            }

            let mut result = SearchResult::new(
                title.trim().to_string(),
                result_url,
                String::new(),
                "emojipedia".to_string(),
            );
            result.engine_rank = Some(i + 1);
            result.category = Some(SearchCategory::General);
            results.push(result);
        }

        Ok(results)
    }
}
