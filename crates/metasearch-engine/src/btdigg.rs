//! BTDigg torrent search engine.
//!
//! Scrapes btdig.com for torrent metadata via HTML parsing.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::MetasearchError,
};

pub struct Btdigg {
    client: Client,
}

impl Btdigg {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Btdigg {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "btdigg".to_string(),
            display_name: "BTDigg".to_string(),
            description: "BitTorrent DHT search engine".to_string(),
            categories: vec![SearchCategory::Files],
            enabled: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let search_term = urlencoding::encode(&query.query);
        let page = query.page.unwrap_or(1) - 1; // 0-indexed
        let url = format!("https://btdig.com/search?q={}&p={}", search_term, page);

        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| MetasearchError::RequestError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let result_sel = Selector::parse("div.one_result").unwrap();
        let name_sel = Selector::parse("div.torrent_name a").unwrap();
        let excerpt_sel = Selector::parse("div.torrent_excerpt").unwrap();
        let size_sel = Selector::parse("span.torrent_size").unwrap();
        let magnet_sel = Selector::parse("div.torrent_magnet a").unwrap();

        let mut results = Vec::new();

        for element in document.select(&result_sel) {
            let title = element.select(&name_sel).next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let href = element.select(&name_sel).next()
                .and_then(|el| el.value().attr("href"))
                .map(|h| {
                    if h.starts_with("http") { h.to_string() }
                    else { format!("https://btdig.com{}", h) }
                })
                .unwrap_or_default();

            let content = element.select(&excerpt_sel).next()
                .map(|el| el.text().collect::<String>().trim().replace('\n', " | "))
                .unwrap_or_default();

            let filesize = element.select(&size_sel).next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let _magnet = element.select(&magnet_sel).next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or_default();

            let snippet = if filesize.is_empty() {
                content.clone()
            } else {
                format!("Size: {} | {}", filesize, content)
            };

            if !title.is_empty() && !href.is_empty() {
                let mut result = SearchResult::new(&title, &href, &snippet, "btdigg");
                result.category = Some(SearchCategory::Files);
                results.push(result);
            }
        }

        Ok(results)
    }
}
