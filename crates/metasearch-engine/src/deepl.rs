//! DeepL — translation engine (API key required)
//!
//! Posts to `https://api-free.deepl.com/v2/translate` with `auth_key`.
//! Requires a DeepL API key. Disabled by default (None api_key).
//!
//! Reference: <https://github.com/searxng/searxng/blob/master/searx/engines/deepl.py>

use async_trait::async_trait;
use reqwest::Client;

use metasearch_core::{
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};

pub struct DeepL {
    client: Client,
    api_key: Option<String>,
}

impl DeepL {
    pub fn new(client: Client, api_key: Option<String>) -> Self {
        Self { client, api_key }
    }
}

#[async_trait]
impl SearchEngine for DeepL {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "DeepL".to_string(),
            display_name: "DeepL".to_string(),
            homepage: "https://api-free.deepl.com".to_string(),
            categories: vec![SearchCategory::General, SearchCategory::General],
            enabled: self.api_key.is_some(),
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let api_key = match &self.api_key {
            Some(k) => k,
            None => return Ok(vec![]),
        };

        let resp = self
            .client
            .post("https://api-free.deepl.com/v2/translate")
            .form(&[
                ("auth_key", api_key.as_str()),
                ("text", &query.query),
                ("target_lang", "EN"),
            ])
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("DeepL request error: {e}")))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("DeepL parse error: {e}")))?;

        let mut results = Vec::new();

        if let Some(translations) = json.get("translations").and_then(|v| v.as_array()) {
            for (i, t) in translations.iter().enumerate() {
                if let Some(text) = t.get("text").and_then(|v| v.as_str()) {
                    let detected = t
                        .get("detected_source_language")
                        .and_then(|v| v.as_str())
                        .unwrap_or("auto");
                    results.push(SearchResult {
                        title: format!("DeepL ({} → EN): {}", detected, text),
                        url: "https://www.deepl.com/translator".to_string(),
                        content: text.to_string(),
                        engine: "deepl".to_string(),
                        engine_rank: (i + 1) as u32,
                    });
                }
            }
        }

        Ok(results)
    }
}
