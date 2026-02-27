//! Hacker News engine — search via Algolia HN Search API.
//! Translated from SearXNG `searx/engines/hackernews.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct HackerNews {
    client: Client,
}

impl HackerNews {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for HackerNews {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "hackernews".to_string(),
            display_name: "Hacker News".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1).saturating_sub(1);

        let url = format!(
            "https://hn.algolia.com/api/v1/search?query={}&page={}&hitsPerPage=30&tags=story",
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

        if let Some(hits) = data["hits"].as_array() {
            for (i, hit) in hits.iter().enumerate() {
                let object_id = hit["objectID"].as_str().unwrap_or_default();
                let title = hit["title"].as_str().unwrap_or("Untitled");
                let points = hit["points"].as_u64().unwrap_or(0);
                let num_comments = hit["num_comments"].as_u64().unwrap_or(0);
                let author = hit["author"].as_str().unwrap_or("");

                let url_str = hit["url"].as_str().unwrap_or("");
                let hn_url = format!("https://news.ycombinator.com/item?id={}", object_id);

                let snippet = format!(
                    "{} — points: {} | comments: {} | by {}",
                    if url_str.is_empty() { &hn_url } else { url_str },
                    points,
                    num_comments,
                    author,
                );

                let mut result = SearchResult::new(
                    title.to_string(),
                    hn_url,
                    snippet,
                    "hackernews".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::IT);
                results.push(result);
            }
        }

        Ok(results)
    }
}
