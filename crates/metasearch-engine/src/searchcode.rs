//! Searchcode — source code search engine
//!
//! Searchcode provides a free JSON API for searching across public
//! code repositories. No API key required.
//!
//! Reference: <https://searchcode.com/api/>

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};

pub struct Searchcode {
    client: Client,
}

impl Searchcode {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Option<Vec<CodeResult>>,
}

#[derive(Debug, Deserialize)]
struct CodeResult {
    url: Option<String>,
    name: Option<String>,
    filename: Option<String>,
    repo: Option<String>,
    lines: Option<serde_json::Value>,
}

#[async_trait]
impl SearchEngine for Searchcode {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "Searchcode".to_string(),
            description: "Source code search engine".to_string(),
            categories: vec![SearchCategory::IT],
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://searchcode.com/api/codesearch_I/?q={}",
            urlencoding::encode(&query.query)
        );

        let resp = self.client.get(&url).send().await?;
        let data: ApiResponse = resp.json().await?;

        let results = data
            .results
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let result_url = item.url?;
                let name = item.name.unwrap_or_default();
                let filename = item.filename.unwrap_or_default();
                let repo = item.repo.unwrap_or_default();

                let title = if filename.is_empty() {
                    name.clone()
                } else {
                    format!("{} — {}", name, filename)
                };

                // Extract first few code lines as content preview
                let content = match &item.lines {
                    Some(serde_json::Value::Object(map)) => {
                        let mut lines: Vec<(i64, String)> = map
                            .iter()
                            .filter_map(|(k, v)| {
                                let line_no: i64 = k.parse().ok()?;
                                let code = v.as_str()?.to_string();
                                Some((line_no, code))
                            })
                            .collect();
                        lines.sort_by_key(|(n, _)| *n);
                        lines
                            .iter()
                            .take(3)
                            .map(|(_, code)| code.trim().to_string())
                            .collect::<Vec<_>>()
                            .join(" | ")
                    }
                    _ => format!("Code in {}", repo),
                };

                Some(SearchResult {
                    title,
                    url: result_url,
                    content,
                    engine: "Searchcode".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail: None,
                })
            })
            .collect();

        Ok(results)
    }
}

