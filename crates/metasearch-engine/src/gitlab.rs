//! GitLab engine — search projects via GitLab REST API.
//! Translated from SearXNG `searx/engines/gitlab.py`.
//!
//! Works with any GitLab instance (gitlab.com, self-hosted, etc.).

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct GitLab {
    client: Client,
    base_url: String,
}

impl GitLab {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: "https://gitlab.com".to_string(),
        }
    }

    pub fn with_base_url(client: Client, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl SearchEngine for GitLab {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "gitlab".to_string(),
            display_name: "GitLab".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 0.8,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);

        let url = format!(
            "{}/api/v4/projects?search={}&page={}",
            self.base_url,
            urlencoding::encode(&query.query),
            page,
        );

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp.json().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(projects) = data.as_array() {
            for (i, item) in projects.iter().enumerate() {
                let name = item["name"].as_str().unwrap_or_default();
                let web_url = item["web_url"].as_str().unwrap_or_default();
                let description = item["description"].as_str().unwrap_or("");
                let stars = item["star_count"].as_u64().unwrap_or(0);
                let forks = item["forks_count"].as_u64().unwrap_or(0);
                let namespace = item["namespace"]["name"].as_str().unwrap_or("");

                let snippet = format!(
                    "{} — ⭐ {} | 🍴 {} | by {}",
                    description, stars, forks, namespace,
                );

                let mut result = SearchResult::new(
                    name.to_string(),
                    web_url.to_string(),
                    snippet,
                    "gitlab".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::IT);
                result.thumbnail = item["avatar_url"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
