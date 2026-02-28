//! SourceHut — search projects on sr.ht.
//!
//! Scrapes HTML search results from sr.ht/projects.
//! No authentication required.
//!
//! Reference: <https://sr.ht>

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

pub struct Sourcehut {
    metadata: EngineMetadata,
    client: Client,
}

impl Sourcehut {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "sourcehut".to_string(),
                display_name: "SourceHut".to_string(),
                homepage: "https://sr.ht".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Sourcehut {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://sr.ht/projects?search={}&page={}&sort=recently-updated",
            urlencoding::encode(&query.query),
            query.page,
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
        let event_sel = Selector::parse("div.event-list div.event")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let h4_sel =
            Selector::parse("h4").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel = Selector::parse("h4 a")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let desc_sel =
            Selector::parse("p").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (rank, event) in document.select(&event_sel).enumerate() {
            let links: Vec<_> = event.select(&link_sel).collect();
            let project_link = if links.len() >= 2 {
                links[1]
            } else if !links.is_empty() {
                links[0]
            } else {
                continue;
            };

            let href = project_link.value().attr("href").unwrap_or_default();
            if href.is_empty() {
                continue;
            }

            let page_url = format!("https://sr.ht{}", href);

            let title = event
                .select(&h4_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            let content = event
                .select(&desc_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            if title.is_empty() {
                continue;
            }

            let mut result =
                SearchResult::new(title, page_url, content, self.metadata.name.clone());
            result.engine_rank = (rank + 1) as u32;
            results.push(result);
        }

        Ok(results)
    }
}
