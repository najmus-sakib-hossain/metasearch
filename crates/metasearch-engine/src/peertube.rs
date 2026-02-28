//! PeerTube — federated video search via search.joinpeertube.org API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde::Deserialize;

pub struct PeerTube {
    metadata: EngineMetadata,
    client: Client,
}

impl PeerTube {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "peertube".to_string(),
                display_name: "PeerTube".to_string(),
                homepage: "https://joinpeertube.org".to_string(),
                categories: vec![SearchCategory::Videos],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    data: Option<Vec<VideoItem>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoItem {
    name: Option<String>,
    url: Option<String>,
    description: Option<String>,
    thumbnail_path: Option<String>,
    account: Option<AccountInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountInfo {
    host: Option<String>,
}

#[async_trait]
impl SearchEngine for PeerTube {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let start = (query.page.saturating_sub(1)) * 10;
        let url = format!(
            "https://search.joinpeertube.org/api/v1/search/videos?search={}&start={}&count=10&sort=-match",
            urlencoding::encode(&query.query),
            start
        );

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("PeerTube request failed: {e}")))?;

        let api: ApiResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("PeerTube parse failed: {e}")))?;

        let results = api
            .data
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let title = item.name?;
                let result_url = item.url?;
                let host = item.account.and_then(|a| a.host).unwrap_or_default();
                let thumbnail_url = item.thumbnail_path.map(|p| {
                    if p.starts_with("http") {
                        p
                    } else {
                        format!("https://{}{}", host, p)
                    }
                });
                Some(SearchResult {
                    title,
                    url: result_url,
                    snippet: item.description.unwrap_or_default(),
                    engine: "peertube".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail_url,
                })
            })
            .collect();

        Ok(results)
    }
}
