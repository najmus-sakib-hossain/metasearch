//! Wikipedia search engine implementation.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::info;

pub struct Wikipedia {
    metadata: EngineMetadata,
    client: Client,
}

impl Wikipedia {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "wikipedia".to_string(),
                display_name: "Wikipedia".to_string(),
                homepage: "https://www.wikipedia.org".to_string(),
                categories: vec![SearchCategory::General, SearchCategory::Science],
                enabled: true,
                timeout_ms: 3000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
struct WikiSearchResponse {
    #[serde(default)]
    query: Option<WikiQuery>,
}

#[derive(Deserialize)]
struct WikiQuery {
    #[serde(default)]
    search: Vec<WikiSearchItem>,
}

#[derive(Deserialize)]
struct WikiSearchItem {
    title: String,
    snippet: String,
    pageid: u64,
}

#[async_trait]
impl SearchEngine for Wikipedia {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let lang = query.language.as_deref().unwrap_or("en");
        let url = format!(
            "https://{}.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&srlimit=10",
            lang,
            urlencoding::encode(&query.query)
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?
            .json::<WikiSearchResponse>()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let results = resp
            .query
            .map(|q| q.search)
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let mut r = SearchResult::new(
                    &item.title,
                    format!("https://{}.wikipedia.org/wiki/{}", lang, item.title.replace(' ', "_")),
                    html_escape::decode_html_entities(&item.snippet).to_string(),
                    "wikipedia",
                );
                r.engine_rank = i as u32;
                r.category = "general".to_string();
                r
            })
            .collect();

        info!(engine = "wikipedia", count = ?resp.query.as_ref().map(|q| q.search.len()), "Search complete");
        Ok(results)
    }
}
