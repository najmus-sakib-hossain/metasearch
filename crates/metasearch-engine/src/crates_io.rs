//! crates.io engine — search Rust crates via crates.io API.
//! Translated from SearXNG `searx/engines/crates.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

const PAGE_SIZE: u32 = 10;

pub struct CratesIo {
    client: Client,
}

impl CratesIo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for CratesIo {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "crates_io".to_string(),
            display_name: "crates.io".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);

        let url = format!(
            "https://crates.io/api/v1/crates?q={}&page={}&per_page={}",
            urlencoding::encode(&query.query),
            page,
            PAGE_SIZE,
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "metasearch-engine/1.0 (https://github.com/najmus-sakib-hossain/metasearch)")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(crates) = data["crates"].as_array() {
            for (i, krate) in crates.iter().enumerate() {
                let name = krate["name"].as_str().unwrap_or_default();
                let description = krate["description"].as_str().unwrap_or("");
                let version = krate["newest_version"].as_str()
                    .or_else(|| krate["max_version"].as_str())
                    .unwrap_or("?");
                let downloads = krate["downloads"].as_u64().unwrap_or(0);

                let crate_url = format!("https://crates.io/crates/{}", name);
                let snippet = format!(
                    "v{} — {} — {} downloads",
                    version, description, downloads,
                );

                let mut result = SearchResult::new(
                    name.to_string(),
                    crate_url,
                    snippet,
                    "crates_io".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::IT);
                results.push(result);
            }
        }

        Ok(results)
    }
}
