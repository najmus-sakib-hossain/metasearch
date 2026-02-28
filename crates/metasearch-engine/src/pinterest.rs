//! Pinterest — image search via Pinterest resource API.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use serde_json::Value;

pub struct Pinterest {
    metadata: EngineMetadata,
    client: Client,
}

impl Pinterest {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "pinterest".to_string(),
                display_name: "Pinterest".to_string(),
                homepage: "https://www.pinterest.com".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Pinterest {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let data = serde_json::json!({
            "options": {
                "query": query.query,
                "page_size": 25,
                "bookmarks": [],
                "field_set_key": "unauth_react",
                "scope": "pins"
            }
        });
        let data_str = serde_json::to_string(&data)
            .map_err(|e| MetasearchError::Engine(format!("Pinterest serialize failed: {e}")))?;

        let url = format!(
            "https://www.pinterest.com/resource/BaseSearchResource/get/?data={}",
            urlencoding::encode(&data_str)
        );

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Pinterest request failed: {e}")))?;

        let json: Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Pinterest parse failed: {e}")))?;

        let empty = vec![];
        let results_arr = json["resource_response"]["data"]["results"]
            .as_array()
            .unwrap_or(&empty);

        let results = results_arr
            .iter()
            .enumerate()
            .filter_map(|(i, pin)| {
                let pin_id = pin["id"].as_str()?;
                let title = pin["title"]
                    .as_str()
                    .or_else(|| pin["grid_title"].as_str())
                    .unwrap_or("Pinterest Pin")
                    .to_string();
                let result_url = format!("https://www.pinterest.com/pin/{}/", pin_id);
                let snippet = pin["description"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let thumbnail_url = pin["images"]["236x"]["url"]
                    .as_str()
                    .or_else(|| pin["images"]["orig"]["url"].as_str())
                    .map(|s| s.to_string());

                Some(SearchResult {
                    title,
                    url: result_url,
                    snippet,
                    engine: "pinterest".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail_url,
                })
            })
            .collect();

        Ok(results)
    }
}
