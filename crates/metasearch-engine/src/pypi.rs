//! PyPI engine — search Python packages via pypi.org HTML scraping.
//! Translated from SearXNG `searx/engines/pypi.py`.

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

pub struct PyPI {
    metadata: EngineMetadata,
    client: Client,
}

impl PyPI {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "pypi".to_string(),
                display_name: "PyPI".to_string(),
                homepage: "https://pypi.org".to_string(),
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
impl SearchEngine for PyPI {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let url = format!(
            "https://pypi.org/search/?q={}&page={}",
            urlencoding::encode(&query.query),
            page,
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let snippet_sel = Selector::parse("a.package-snippet").unwrap();
        let name_sel = Selector::parse("span.package-snippet__name").unwrap();
        let version_sel = Selector::parse("span.package-snippet__version").unwrap();
        let desc_sel = Selector::parse("p.package-snippet__description").unwrap();

        let mut results = Vec::new();

        for (i, el) in document.select(&snippet_sel).enumerate() {
            let href = el.value().attr("href").unwrap_or("");
            let package_url = format!("https://pypi.org{}", href);

            let name = el
                .select(&name_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let version = el
                .select(&version_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let description = el
                .select(&desc_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let snippet = format!("v{} — {}", version.trim(), description.trim());

            let mut result = SearchResult::new(
                name.trim().to_string(),
                package_url,
                snippet,
                "pypi".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::IT.to_string();
            results.push(result);
        }

        Ok(results)
    }
}
