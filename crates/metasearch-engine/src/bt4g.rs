//! BT4G torrent search engine.
//!
//! Searches bt4gprx.com via its RSS/XML API for torrent metadata.

use async_trait::async_trait;
use reqwest::Client;
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Bt4g {
    client: Client,
}

impl Bt4g {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Bt4g {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "bt4g".to_string(),
            display_name: "BT4G".to_string(),
            description: "Torrent metadata search engine".to_string(),
            categories: vec![SearchCategory::Files],
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let search_term = urlencoding::encode(&query.query);
        let page = query.page.unwrap_or(1);
        let url = format!(
            "https://bt4gprx.com/search?q={}&orderby=relevance&category=all&p={}&page=rss",
            search_term, page
        );

        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let mut results = Vec::new();

        // Simple XML parsing for RSS items
        let mut pos = 0;
        while let Some(item_start) = body[pos..].find("<item>") {
            let item_start = pos + item_start;
            let item_end = match body[item_start..].find("</item>") {
                Some(e) => item_start + e + 7,
                None => break,
            };
            let item = &body[item_start..item_end];

            let title = extract_tag(item, "title").unwrap_or_default();
            let link = extract_tag(item, "guid").unwrap_or_default();
            let description = extract_tag(item, "description").unwrap_or_default();

            // Description contains file info separated by &lt;br&gt;
            let snippet = html_escape::decode_html_entities(&description)
                .to_string()
                .replace("<br>", " | ");

            if !title.is_empty() && !link.is_empty() {
                let mut result = SearchResult::new(&title, &link, &snippet, "bt4g");
                result.category = Some(SearchCategory::Files);
                results.push(result);
            }

            pos = item_end;
        }

        Ok(results)
    }
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    let content = &xml[start..end];
    // Handle CDATA
    if content.starts_with("<![CDATA[") && content.ends_with("]]>") {
        Some(content[9..content.len() - 3].to_string())
    } else {
        Some(content.to_string())
    }
}
