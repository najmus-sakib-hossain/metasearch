//! pkg.go.dev — Go package search via HTML scraping.

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

pub struct PkgGoDev {
    metadata: EngineMetadata,
    client: Client,
}

impl PkgGoDev {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "pkg_go_dev".to_string(),
                display_name: "pkg.go.dev".to_string(),
                homepage: "https://pkg.go.dev".to_string(),
                categories: vec![SearchCategory::IT],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for PkgGoDev {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let url = format!(
            "https://pkg.go.dev/search?q={}&m=package&page={}",
            urlencoding::encode(&query.query),
            page
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| MetasearchError::Engine(format!("pkg.go.dev request failed: {e}")))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::Engine(format!("pkg.go.dev read failed: {e}")))?;

        let document = Html::parse_document(&html_text);
        let result_sel = Selector::parse(".SearchSnippet").unwrap();
        let link_sel = Selector::parse("a[href]").unwrap();
        let synopsis_sel = Selector::parse(".SearchSnippet-synopsis").unwrap();
        let header_sel = Selector::parse(".SearchSnippet-header-path").unwrap();

        let mut results = Vec::new();
        for (i, el) in document.select(&result_sel).enumerate() {
            let title = el
                .select(&header_sel)
                .next()
                .map(|h| h.text().collect::<String>().trim().to_string())
                .or_else(|| {
                    el.select(&link_sel)
                        .next()
                        .map(|a| a.text().collect::<String>().trim().to_string())
                })
                .unwrap_or_default();

            let href = el
                .select(&link_sel)
                .next()
                .and_then(|a| a.value().attr("href"))
                .unwrap_or_default();

            let result_url = if href.starts_with("http") {
                href.to_string()
            } else {
                format!("https://pkg.go.dev{}", href)
            };

            let snippet = el
                .select(&synopsis_sel)
                .next()
                .map(|s| s.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if !title.is_empty() {
                results.push(SearchResult {
                    title,
                    url: result_url,
                    content,
                    engine: "pkg_go_dev".to_string(),
                    engine_rank: (i + 1) as u32,
                    thumbnail: None,
                });
            }
        }

        Ok(results)
    }
}
