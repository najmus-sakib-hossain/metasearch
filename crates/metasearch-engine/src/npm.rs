//! npm engine — search JavaScript packages via npms.io API.
//! Translated from SearXNG `searx/engines/npm.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

const PAGE_SIZE: u32 = 25;

pub struct Npm {
    client: Client,
}

impl Npm {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Npm {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "npm".to_string(),
            display_name: "npm".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let from = (page as u32 - 1) * PAGE_SIZE;

        let url = format!(
            "https://api.npms.io/v2/search?q={}&from={}&size={}",
            urlencoding::encode(&query.query),
            from,
            PAGE_SIZE,
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(entries) = data["results"].as_array() {
            for (i, entry) in entries.iter().enumerate() {
                let package = &entry["package"];
                let name = package["name"].as_str().unwrap_or_default();
                let description = package["description"].as_str().unwrap_or("");
                let version = package["version"].as_str().unwrap_or("");
                let npm_url = package["links"]["npm"].as_str().unwrap_or_default();
                let author = package["author"]["name"].as_str().unwrap_or("");

                let snippet = format!(
                    "v{} — {} — by {}",
                    version, description, author,
                );

                let mut result = SearchResult::new(
                    name.to_string(),
                    npm_url.to_string(),
                    snippet,
                    "npm".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::IT);
                results.push(result);
            }
        }

        Ok(results)
    }
}
