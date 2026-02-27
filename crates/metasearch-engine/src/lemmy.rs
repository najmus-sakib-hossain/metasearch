//! Lemmy engine — search posts on Lemmy (federated Reddit alternative) via API.
//! Translated from SearXNG `searx/engines/lemmy.py`.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Lemmy {
    client: Client,
    base_url: String,
}

impl Lemmy {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: "https://lemmy.ml".to_string(),
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
impl SearchEngine for Lemmy {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "lemmy".to_string(),
            display_name: "Lemmy".to_string(),
            categories: vec![SearchCategory::SocialMedia],
            enabled: true,
            weight: 0.8,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);

        let url = format!(
            "{}/api/v3/search?q={}&page={}&type_=Posts",
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

        if let Some(posts) = data["posts"].as_array() {
            for (i, item) in posts.iter().enumerate() {
                let post = &item["post"];
                let counts = &item["counts"];
                let creator = &item["creator"];
                let community = &item["community"];

                let title = post["name"].as_str().unwrap_or_default();
                let post_url = post["ap_id"].as_str().unwrap_or_default();
                let body = post["body"].as_str().unwrap_or("");
                let user = creator["display_name"].as_str()
                    .or_else(|| creator["name"].as_str())
                    .unwrap_or("");
                let community_title = community["title"].as_str().unwrap_or("");
                let upvotes = counts["upvotes"].as_u64().unwrap_or(0);
                let downvotes = counts["downvotes"].as_u64().unwrap_or(0);
                let comments = counts["comments"].as_u64().unwrap_or(0);

                let content = if body.len() > 300 {
                    format!("{}...", &body[..300])
                } else {
                    body.to_string()
                };

                let snippet = format!(
                    "{} — ▲ {} ▼ {} | 💬 {} | by {} | {}",
                    content, upvotes, downvotes, comments, user, community_title,
                );

                let mut result = SearchResult::new(
                    title.to_string(),
                    post_url.to_string(),
                    snippet,
                    "lemmy".to_string(),
                );
                result.engine_rank = Some(i + 1);
                result.category = Some(SearchCategory::SocialMedia);
                result.thumbnail = post["thumbnail_url"].as_str().map(|s| s.to_string());
                results.push(result);
            }
        }

        Ok(results)
    }
}
