//! Google search engine implementation.
//!
//! Scrapes Google's web search results page. Based on SearXNG's `google.py`.
//! Uses async API with arc_id for better bot detection avoidance.

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
use tracing::info;
use smallvec::smallvec;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, distributions::Alphanumeric};

// Global arc_id cache (regenerated every hour)
static ARC_ID_CACHE: Mutex<Option<(String, u64)>> = Mutex::new(None);

pub struct Google {
    metadata: EngineMetadata,
    client: Client,
}

impl Google {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "google".to_string().into(),
                display_name: "Google".to_string().into(),
                homepage: "https://www.google.com".to_string().into(),
                categories: smallvec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.5,
            },
            client,
        }
    }

    /// Map safe_search level to Google's `safe` parameter value.
    fn safe_search_param(level: u8) -> &'static str {
        match level {
            2 => "active",
            1 => "medium",
            _ => "off",
        }
    }
    
    /// Generate arc_id for async API (regenerated every hour like SearXNG)
    fn generate_arc_id(start: u32) -> String {
        let mut cache = ARC_ID_CACHE.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Regenerate every hour
        if cache.is_none() || (now - cache.as_ref().unwrap().1) > 3600 {
            let random_id: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(23)
                .map(char::from)
                .collect();
            *cache = Some((random_id, now));
        }
        
        let random_id = &cache.as_ref().unwrap().0;
        format!("arc_id:srp_{}_1{:02},use_ac:true,_fmt:prog", random_id, start)
    }
}

#[async_trait]
impl SearchEngine for Google {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let start = (query.page.max(1) - 1) * 10;
        let lang = query.language.as_deref().unwrap_or("en");
        let safe = Self::safe_search_param(query.safe_search);

        // Generate arc_id for async API (like SearXNG)
        let async_param = Self::generate_arc_id(start);

        let url = format!(
            "https://www.google.com/search?q={}&start={}&hl={}&lr=lang_{}&ie=utf8&oe=utf8&filter=0&safe={}&asearch=arc&async={}",
            urlencoding::encode(&query.query),
            start,
            lang,
            lang,
            safe,
            urlencoding::encode(&async_param),
        );

        let resp = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .header("Accept", "*/*")
            .header("Accept-Language", format!("{},en-US;q=0.9,en;q=0.8", lang))
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Referer", "https://www.google.com/")
            .header("Cookie", "CONSENT=YES+")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let body = resp
            .text()
            .await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let mut results = Vec::new();

        // SearXNG uses: .//div[contains(@class, "MjjYud")]
        let result_selector = Selector::parse("div.MjjYud").unwrap();
        let title_sel = Selector::parse("div[role='link'], h3").unwrap();
        let link_sel = Selector::parse("a[href]").unwrap();
        let snippet_sel = Selector::parse("div[data-sncf='1'], div.VwiC3b, span.aCOpRe, div.s, div.IsZvec").unwrap();

        for (i, element) in document.select(&result_selector).enumerate() {
            // Get the link
            let link_el = match element.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };
            
            let href = link_el.value().attr("href").unwrap_or_default();
            if href.is_empty() || href.starts_with('/') || href.starts_with('#') {
                continue;
            }

            // Handle Google redirect URLs: /url?q=...
            let result_url = if href.contains("/url?q=") {
                href.split("/url?q=")
                    .nth(1)
                    .and_then(|s| s.split('&').next())
                    .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
                    .unwrap_or_else(|| href.to_string())
            } else {
                href.to_string()
            };

            if !result_url.starts_with("http") {
                continue;
            }

            // Get title
            let title: String = element
                .select(&title_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if title.is_empty() {
                continue;
            }

            // Get snippet
            let snippet: String = element
                .select(&snippet_sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            let mut r = SearchResult::new(&title, &result_url, &snippet, "google");
            r.engine_rank = i as u32;
            r.category = SearchCategory::General.to_string();
            results.push(r);

            if results.len() >= 10 {
                break;
            }
        }

        info!(engine = "google", count = results.len(), "Search complete");
        Ok(results)
    }
}
