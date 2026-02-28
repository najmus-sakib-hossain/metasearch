//! Wolfram|Alpha search engine (no API key required).
//! Two-step process: get token from code endpoint, then query with token.
//! Features: No pagination

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use tracing::info;

pub struct WolframAlphaNoapi {
    metadata: EngineMetadata,
    client: Client,
}

impl WolframAlphaNoapi {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "wolframalpha_noapi".to_string(),
                display_name: "Wolfram|Alpha".to_string(),
                homepage: "https://www.wolframalpha.com".to_string(),
                categories: vec![SearchCategory::Science],
                enabled: true,
                timeout_ms: 10000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for WolframAlphaNoapi {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Step 1: Get proxy token
        let token_url =
            "https://www.wolframalpha.com/input/api/v1/code?ts=9999999999999999999";

        let token_resp = self
            .client
            .get(token_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let token_data: serde_json::Value = token_resp
            .json()
            .await
            .map_err(|e| {
                MetasearchError::ParseError(format!("Token JSON error: {}", e))
            })?;

        let token = token_data["code"]
            .as_str()
            .ok_or_else(|| {
                MetasearchError::ParseError(
                    "Missing 'code' in token response".to_string(),
                )
            })?;

        let encoded_query = urlencoding::encode(&query.query);
        let referer = format!(
            "https://www.wolframalpha.com/input/?i={}",
            encoded_query
        );

        // Step 2: Query with token
        let query_url = format!(
            "https://www.wolframalpha.com/input/json.jsp\
             ?async=false&banners=raw&format=image,plaintext\
             &input={}&output=JSON&proxycode={}",
            encoded_query,
            urlencoding::encode(token),
        );

        let resp = self
            .client
            .get(&query_url)
            .header("Referer", &referer)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::ParseError(format!("JSON error: {}", e)))?;

        let result_url = format!(
            "https://www.wolframalpha.com/input/?i={}",
            encoded_query
        );

        let mut results = Vec::new();

        let pods = match data["queryresult"]["pods"].as_array() {
            Some(pods) => pods,
            None => return Ok(results),
        };

        for (i, pod) in pods.iter().enumerate() {
            let title = pod["title"].as_str().unwrap_or_default();
            if title.is_empty() {
                continue;
            }

            let content = pod["subpods"]
                .as_array()
                .and_then(|s| s.first())
                .and_then(|sp| sp["plaintext"].as_str())
                .unwrap_or_default();

            if content.is_empty() {
                continue;
            }

            let mut r =
                SearchResult::new(title, &result_url, content, "wolframalpha_noapi");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::Science.to_string();
            results.push(r);
        }

        info!(
            engine = "wolframalpha_noapi",
            count = results.len(),
            "Search complete"
        );
        Ok(results)
    }
}
