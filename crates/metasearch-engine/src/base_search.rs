//! BASE (Bielefeld Academic Search Engine) — scholarly publications via XML API.
//!
//! Reference: <https://base-search.net>
//! API docs: <https://api.base-search.net/>

use async_trait::async_trait;
use reqwest::Client;
use regex::Regex;
use metasearch_core::{
    engine::{EngineMetadata, SearchEngine},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct BaseSearch {
    client: Client,
}

impl BaseSearch {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for BaseSearch {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "BASE".to_string(),
            description: "Bielefeld Academic Search Engine — scholarly publications".to_string(),
            categories: vec![SearchCategory::General],
            base_url: "https://base-search.net".to_string(),
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let hits: u32 = 10;
        let offset = (page as u32 - 1) * hits;

        let url = format!(
            "https://api.base-search.net/cgi-bin/BaseHttpSearchInterface.fcgi?func=PerformSearch&query={}&boost=oa&hits={}&offset={}",
            urlencoding::encode(&query.query),
            hits,
            offset
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "metasearch/1.0")
            .send()
            .await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let mut results = Vec::new();

        // Parse XML: each result is in <doc> with <str name="...">value</str>
        let doc_re = Regex::new(r"(?s)<doc>(.*?)</doc>")
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;
        let field_re = Regex::new(r#"(?s)<str name="([^"]+)">(.*?)</str>"#)
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        for doc_cap in doc_re.captures_iter(&body) {
            let doc_content = &doc_cap[1];
            let mut title = String::new();
            let mut link = String::new();
            let mut content = String::new();

            for field_cap in field_re.captures_iter(doc_content) {
                let name = &field_cap[1];
                let value = html_escape::decode_html_entities(&field_cap[2]).to_string();
                match name {
                    "dctitle" => title = value,
                    "dclink" => link = value,
                    "dcdescription" => {
                        content = if value.len() > 300 {
                            format!("{}...", &value[..300])
                        } else {
                            value
                        };
                    }
                    _ => {}
                }
            }

            if !link.is_empty() && !title.is_empty() {
                let mut result = SearchResult::new(
                    title,
                    link,
                    content,
                    "BASE".to_string(),
                );
                result.category = Some(SearchCategory::General);
                results.push(result);
            }
        }

        Ok(results)
    }
}
