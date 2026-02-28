//! Mwmbl — non-profit, ad-free search engine.
//!
//! Queries the Mwmbl JSON API.

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

pub struct Mwmbl {
    metadata: EngineMetadata,
    client: Client,
}

impl Mwmbl {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "mwmbl".to_string(),
                display_name: "Mwmbl".to_string(),
                homepage: "https://mwmbl.org".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
struct TitlePart {
    value: String,
}

#[derive(Deserialize)]
struct ExtractPart {
    value: String,
}

#[derive(Deserialize)]
struct ApiResult {
    url: String,
    title: Vec<TitlePart>,
    extract: Vec<ExtractPart>,
}

#[async_trait]
impl SearchEngine for Mwmbl {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://api.mwmbl.org/api/v1/search/?s={}",
            urlencoding::encode(&query.query)
        );

        let items: Vec<ApiResult> = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let results = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let title = item.title.into_iter().map(|t| t.value).collect::<String>();
                let snippet = item
                    .extract
                    .into_iter()
                    .next()
                    .map(|e| e.value)
                    .unwrap_or_default();
                let mut r = SearchResult::new(
                    title,
                    item.url,
                    snippet,
                    self.metadata.name.clone(),
                );
                r.engine_rank = (i + 1) as u32;
                r
            })
            .collect();

        Ok(results)
    }
}
