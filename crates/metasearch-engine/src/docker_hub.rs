//! Docker Hub engine — search container images via Docker Hub API.
//! Translated from SearXNG `searx/engines/docker_hub.py`.

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

pub struct DockerHub {
    client: Client,
}

impl DockerHub {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for DockerHub {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "docker_hub".to_string(),
            display_name: "Docker Hub".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let from = PAGE_SIZE * (page as u32 - 1);

        let url = format!(
            "https://hub.docker.com/api/search/v3/catalog/search?query={}&from={}&size={}",
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

        if let Some(items) = data["results"].as_array() {
            for (i, item) in items.iter().enumerate() {
                let name = item["name"].as_str().unwrap_or_default();
                let slug = item["slug"].as_str().unwrap_or_default();
                let description = item["short_description"].as_str().unwrap_or("");
                let source = item["source"].as_str().unwrap_or("");
                let stars = item["star_count"].as_u64().unwrap_or(0);

                let is_official = source == "store" || source == "official";
                let prefix = if is_official { "/_/" } else { "/r/" };
                let page_url = format!("https://hub.docker.com{}{}", prefix, slug);

                let snippet = format!("{} — ⭐ {} stars", description, stars);

                let mut result = SearchResult::new(
                    name.to_string(),
                    page_url,
                    snippet,
                    "docker_hub".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::IT);
                results.push(result);
            }
        }

        Ok(results)
    }
}
