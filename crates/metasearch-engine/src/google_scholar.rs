//! Google Scholar search engine.
//! Translated from SearXNG's `google_scholar.py`.
//! HTML scraping from scholar.google.com.
//! No API key required.
//! Features: Paging

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
use tracing::{info, warn};

pub struct GoogleScholar {
    metadata: EngineMetadata,
    client: Client,
}

impl GoogleScholar {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "google_scholar".to_string(),
                display_name: "Google Scholar".to_string(),
                homepage: "https://scholar.google.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for GoogleScholar {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let start = (query.page.max(1) - 1) * 10;
        let url = format!(
            "https://scholar.google.com/scholar?q={}&start={}&as_sdt=2007&as_vis=0",
            urlencoding::encode(&query.query),
            start
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        // Check for captcha
        if text.contains("gs_captcha_f") || text.contains("/sorry/index") {
            warn!(engine = "google_scholar", "Captcha detected");
            return Err(MetasearchError::EngineError {
                engine: "google_scholar".to_string(),
                message: "Google Scholar captcha detected".to_string(),
            });
        }

        let document = Html::parse_document(&text);
        let result_sel = Selector::parse("div[data-rp]").unwrap();
        let title_link_sel = Selector::parse("h3 a").unwrap();
        let content_sel = Selector::parse("div.gs_rs").unwrap();
        let meta_sel = Selector::parse("div.gs_a").unwrap();

        let mut results = Vec::new();
        for (i, result_el) in document.select(&result_sel).enumerate() {
            // Get title and URL from h3 > a
            let title_el = match result_el.select(&title_link_sel).next() {
                Some(el) => el,
                None => continue,
            };
            let title: String = title_el.text().collect();
            let href = title_el.value().attr("href").unwrap_or("");
            if title.is_empty() || href.is_empty() {
                continue;
            }

            // Get abstract / content
            let content: String = result_el
                .select(&content_sel)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            // Get metadata line (authors - journal, year - publisher)
            let meta: String = result_el
                .select(&meta_sel)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let snippet = if content.is_empty() { &meta } else { &content };

            let mut r = SearchResult::new(&title, href, snippet, "google_scholar");
            r.engine_rank = (i + 1) as u32;
            r.category = SearchCategory::General;

            // Try to extract year from metadata (e.g., "Author - Journal, 2024 - Publisher")
            if let Some(caps) = regex::Regex::new(r"\b(19|20)\d{2}\b")
                .ok()
                .and_then(|re| re.find(&meta))
            {
                if let Ok(year) = caps.as_str().parse::<i32>() {
                    if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, 1, 1) {
                        r.published_date = date.and_hms_opt(0, 0, 0).map(|ndt| {
                            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                                ndt,
                                chrono::Utc,
                            )
                        });
                    }
                }
            }

            results.push(r);
        }

        info!(
            engine = "google_scholar",
            count = results.len(),
            "Search complete"
        );
        Ok(results)
    }
}
