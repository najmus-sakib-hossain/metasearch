//! Reddit engine — search posts via Reddit JSON API.
//! Translated from SearXNG `searx/engines/reddit.py`.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct Reddit {
    metadata: EngineMetadata,
    client: Client,
}

impl Reddit {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "reddit".to_string(),
                display_name: "Reddit".to_string(),
                homepage: "https://www.reddit.com".to_string(),
                categories: vec![SearchCategory::SocialMedia],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Reddit {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://www.reddit.com/search.json?q={}&limit=25",
            urlencoding::encode(&query.query),
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "metasearch-engine/1.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        if let Some(children) = data["data"]["children"].as_array() {
            for (i, child) in children.iter().enumerate() {
                let post = &child["data"];
                let title = post["title"].as_str().unwrap_or_default();
                let permalink = post["permalink"].as_str().unwrap_or("");
                let subreddit = post["subreddit_name_prefixed"].as_str().unwrap_or("");
                let score = post["score"].as_i64().unwrap_or(0);
                let num_comments = post["num_comments"].as_u64().unwrap_or(0);
                let selftext = post["selftext"].as_str().unwrap_or("");

                let post_url = format!("https://www.reddit.com{}", permalink);

                let content = if selftext.len() > 300 {
                    format!("{}...", &selftext[..300])
                } else {
                    selftext.to_string()
                };

                let snippet = format!(
                    "{} — ⬆ {} | 💬 {} | {}",
                    content, score, num_comments, subreddit,
                );

                let mut result =
                    SearchResult::new(title.to_string(), post_url, snippet, "reddit".to_string());
                result.engine_rank = (i + 1) as u32;
                result.category = SearchCategory::SocialMedia.to_string();
                result.thumbnail = post["thumbnail"]
                    .as_str()
                    .filter(|t| t.starts_with("http"))
                    .map(|t| t.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
