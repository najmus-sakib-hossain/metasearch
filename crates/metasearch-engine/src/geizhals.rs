//! Geizhals — European price-comparison search engine.
//!
//! Scrapes HTML results from geizhals.de.

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::{MetasearchError, Result},
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};

pub struct Geizhals {
    metadata: EngineMetadata,
    client: Client,
}

impl Geizhals {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "geizhals".to_string(),
                display_name: "Geizhals".to_string(),
                homepage: "https://geizhals.de".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 8000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Geizhals {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!(
            "https://geizhals.de/?fs={}&hloc=at&hloc=de&in=",
            urlencoding::encode(&query.query)
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let item_sel = Selector::parse("div.listview__item, article.listview__item")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel = Selector::parse("a.listview__name-link")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let price_sel = Selector::parse("span.price")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let img_sel =
            Selector::parse("img").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (rank, item) in document.select(&item_sel).enumerate() {
            let link_el = match item.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title = link_el.text().collect::<String>().trim().to_string();
            if title.is_empty() {
                continue;
            }

            let href = match link_el.value().attr("href") {
                Some(h) => {
                    if h.starts_with("http") {
                        h.to_string()
                    } else {
                        format!("https://geizhals.de{h}")
                    }
                }
                None => continue,
            };

            let price = item
                .select(&price_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let snippet = if price.is_empty() {
                String::new()
            } else {
                format!("Price: {price}")
            };

            let thumbnail = item
                .select(&img_sel)
                .next()
                .and_then(|el| el.value().attr("src"))
                .map(|s| s.to_string());

            let mut result = SearchResult::new(title, href, snippet, self.metadata.name.clone());
            result.engine_rank = (rank + 1) as u32;
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
