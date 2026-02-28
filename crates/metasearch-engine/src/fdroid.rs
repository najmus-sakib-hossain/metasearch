//! F-Droid engine — search FOSS Android apps via HTML scraping.
//! Translated from SearXNG `searx/engines/fdroid.py`.

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

pub struct Fdroid {
    metadata: EngineMetadata,
    client: Client,
}

impl Fdroid {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "fdroid".to_string(),
                display_name: "F-Droid".to_string(),
                homepage: "https://search.f-droid.org".to_string(),
                categories: vec![SearchCategory::Files],
                enabled: true,
                timeout_ms: 5000,
                weight: 0.7,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Fdroid {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let url = format!(
            "https://search.f-droid.org/?q={}&page={}&lang=",
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
        let app_sel = Selector::parse("a.package-header").unwrap();
        let name_sel = Selector::parse("div h4.package-name").unwrap();
        let summary_sel = Selector::parse("div div span.package-summary").unwrap();
        let license_sel = Selector::parse("div div span.package-license").unwrap();
        let icon_sel = Selector::parse("img.package-icon").unwrap();

        let mut results = Vec::new();

        for (i, app) in document.select(&app_sel).enumerate() {
            let app_url = app.value().attr("href").unwrap_or_default().to_string();

            let title = app
                .select(&name_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            if title.is_empty() {
                continue;
            }

            let summary = app
                .select(&summary_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let license = app
                .select(&license_sel)
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let snippet = format!("{} - {}", summary.trim(), license.trim());

            let thumbnail = app
                .select(&icon_sel)
                .next()
                .and_then(|e| e.value().attr("src"))
                .map(|s| s.to_string());

            let mut result = SearchResult::new(
                title.trim().to_string(),
                app_url,
                snippet.trim().to_string(),
                "fdroid".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::Files.to_string();
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
