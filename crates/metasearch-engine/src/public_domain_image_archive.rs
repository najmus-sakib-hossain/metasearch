//! Public Domain Image Archive — search public domain images.
//!
//! Scrapes HTML search results from pdimagearchive.org.
//!
//! Reference: <https://pdimagearchive.org>

use async_trait::async_trait;
use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};
use reqwest::Client;
use scraper::{Html, Selector};

pub struct PublicDomainImageArchive {
    metadata: EngineMetadata,
    client: Client,
}

impl PublicDomainImageArchive {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "public_domain_image_archive".to_string(),
                display_name: "PDIA".to_string(),
                homepage: "https://pdimagearchive.org".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for PublicDomainImageArchive {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://pdimagearchive.org/?s={}",
            urlencoding::encode(&query.query),
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

        // Try article elements first, then fall back to div.gallery items
        let article_sel = Selector::parse("article")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel =
            Selector::parse("a").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let img_sel =
            Selector::parse("img").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (rank, article) in document.select(&article_sel).enumerate() {
            let link_el = match article.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or_default();
            if href.is_empty() || !href.starts_with("http") {
                continue;
            }

            let img_el = article.select(&img_sel).next();
            let thumbnail = img_el.and_then(|el| el.value().attr("src")).map(|s| s.to_string());
            let title = img_el
                .and_then(|el| el.value().attr("alt"))
                .or_else(|| link_el.value().attr("title"))
                .unwrap_or("Public Domain Image")
                .to_string();

            let mut result = SearchResult::new(
                title,
                href.to_string(),
                String::new(),
                "public_domain_image_archive",
            );
            result.engine_rank = (rank + 1) as u32;
            result.category = SearchCategory::Images.to_string();
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
