//! MediaWiki Action API search — configurable instance URL.
//! SearXNG equivalent: `mediawiki.py`
//!
//! Queries any MediaWiki wiki via the standard Action API `list=search`.
//! Configure `base_url` to point to the wiki root (e.g. `https://en.wikipedia.org`).

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

pub struct MediaWikiEngine {
    client: Client,
    base_url: String,
}

impl MediaWikiEngine {
    pub fn new(client: Client, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }
}

#[derive(Deserialize)]
struct MwApiResponse {
    query: Option<MwQuery>,
}

#[derive(Deserialize)]
struct MwQuery {
    search: Option<Vec<MwSearchResult>>,
}

#[derive(Deserialize)]
struct MwSearchResult {
    title: String,
    content: Option<String>,
    timestamp: Option<String>,
}

/// Strip simple HTML tags from MediaWiki search snippets.
fn strip_search_highlight(s: &str) -> String {
    s.replace("<span class=\"searchmatch\">", "")
        .replace("</span>", "")
}

#[async_trait]
impl SearchEngine for MediaWikiEngine {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "MediaWiki".to_string(),
            display_name: "MediaWiki".to_string(),
            categories: vec![metasearch_core::category::SearchCategory::General],
            enabled: !self.base_url.is_empty(),
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        if self.base_url.is_empty() {
            return Ok(Vec::new());
        }

        let offset = (query.page - 1) * 5;
        let url = format!(
            "{}/w/api.php?action=query&list=search&format=json&srsearch={}&sroffset={}&srlimit=10&srwhat=text&srprop=snippet%7Ctimestamp&srenablerewrites=1",
            self.base_url,
            urlencoding::encode(&query.query),
            offset,
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("MediaWiki: {e}")))?;

        let data: MwApiResponse = resp
            .json()
            .await
            .map_err(|e| MetasearchError::Engine(format!("MediaWiki JSON: {e}")))?;

        let items = data.query.and_then(|q| q.search).unwrap_or_default();

        let mut results = Vec::new();
        for (i, item) in items.iter().enumerate() {
            let title = item.title.clone();
            let page_url = format!(
                "{}/wiki/{}",
                self.base_url,
                urlencoding::encode(&title.replace(' ', "_")),
            );
            let content = item
                .snippet
                .as_deref()
                .map(strip_search_highlight)
                .unwrap_or_default();

            results.push(SearchResult {
                title,
                url: page_url,
                content,
                engine: "MediaWiki".to_string(),
                engine_rank: (i + 1) as u32,
                    score: 0.0,
                    thumbnail: None,
                    published_date: None,
                    category: String::new(),
                    metadata: serde_json::Value::Null,
                });
        }
        Ok(results)
    }
}
