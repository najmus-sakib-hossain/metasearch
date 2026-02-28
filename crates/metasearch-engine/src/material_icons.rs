//! Material Icons — Google Material Symbols icon search.
//!
//! Fetches the full icon metadata JSON from Google Fonts and filters locally.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;

pub struct MaterialIcons {
    metadata: EngineMetadata,
    client: Client,
}

impl MaterialIcons {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "material_icons".to_string(),
                display_name: "Material Icons".to_string(),
                homepage: "https://fonts.google.com/icons".to_string(),
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
impl SearchEngine for MaterialIcons {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let resp = self
            .client
            .get("https://fonts.google.com/metadata/icons?key=material_symbols&incomplete=true")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        // Response starts with ")]}'\n" prefix; skip first 5 chars
        let json_str = if body.len() > 5 { &body[5..] } else { &body };

        let json: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let query_lower = query.query.to_lowercase();
        let query_parts: Vec<&str> = query_lower.split_whitespace().collect();

        let mut results = Vec::new();

        if let Some(icons) = json.get("icons").and_then(|v| v.as_array()) {
            for icon in icons {
                let name = icon
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let tags: Vec<&str> = icon
                    .get("tags")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect()
                    })
                    .unwrap_or_default();
                let categories: Vec<&str> = icon
                    .get("categories")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .collect()
                    })
                    .unwrap_or_default();

                let matches = query_parts.iter().any(|part| {
                    name.contains(part)
                        || tags.iter().any(|t| t.contains(part))
                        || categories.iter().any(|c| c.contains(part))
                });

                if !matches {
                    continue;
                }

                let url = format!(
                    "https://fonts.google.com/icons?icon.query={}&selected=Material+Symbols+Outlined:{name}:FILL@0;wght@400;GRAD@0;opsz@24",
                    urlencoding::encode(name)
                );
                let img_src = format!(
                    "https://fonts.gstatic.com/s/i/short-term/release/materialsymbolsoutlined/{name}/default/24px.svg"
                );

                let snippet = format!(
                    "Tags: {} / Categories: {}",
                    tags.join(", "),
                    categories.join(", ")
                );

                let mut result = SearchResult::new(
                    name.replace('_', " "),
                    url,
                    snippet,
                    self.metadata.name.clone(),
                );
                result.engine_rank = (results.len() + 1) as u32;
                result.thumbnail = Some(img_src);
                result.category = SearchCategory::Images.to_string();
                results.push(result);

                if results.len() >= 20 {
                    break;
                }
            }
        }

        Ok(results)
    }
}
