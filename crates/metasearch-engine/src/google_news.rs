//! Google News search — HTML scraping with base64 URL decoding.
//! SearXNG equivalent: `google_news.py`

use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose;
use reqwest::Client;
use scraper::{Html, Selector};

use metasearch_core::{
    category::SearchCategory,
    engine::{EngineMetadata, SearchEngine},
    error::MetasearchError,
    query::SearchQuery,
    result::SearchResult,
};

pub struct GoogleNews {
    client: Client,
}

impl GoogleNews {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

/// Decode a Google News internal link to the actual article URL.
/// Google News encodes article URLs in base64 within the href path.
fn decode_google_news_url(href: &str) -> Option<String> {
    let path = href.split('?').next()?;
    let encoded = path.rsplit('/').next()?;
    if encoded.is_empty() {
        return None;
    }
    // Add proper base64 padding
    let remainder = encoded.len() % 4;
    let padded = if remainder > 0 {
        format!("{}{}", encoded, "=".repeat(4 - remainder))
    } else {
        encoded.to_string()
    };
    let bytes = general_purpose::URL_SAFE.decode(padded.as_bytes()).ok()?;
    // Find "http" in decoded bytes
    let http_pos = bytes.windows(4).position(|w| w == b"http")?;
    let url_bytes = &bytes[http_pos..];
    // URL ends at 0xd2 byte or end of data
    let end = url_bytes
        .iter()
        .position(|&b| b == 0xd2)
        .unwrap_or(url_bytes.len());
    String::from_utf8(url_bytes[..end].to_vec()).ok()
}

#[async_trait]
impl SearchEngine for GoogleNews {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "Google News".to_string(),
            display_name: "Google News".to_string(),
            homepage: "https://Google News.com".to_string(),
            categories: vec![metasearch_core::category::SearchCategory::News],
            enabled: true,
            timeout_ms: 5000,
            weight: 1.0,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let url = format!(
            "https://news.google.com/search?q={}&hl=en&gl=US&ceid=US:en",
            urlencoding::encode(&query.query),
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
            .map_err(|e| MetasearchError::Engine(format!("Google News: {e}")))?;

        let text = resp
            .text()
            .await
            .map_err(|e| MetasearchError::Engine(format!("Google News body: {e}")))?;

        let doc = Html::parse_document(&text);
        let article_sel = Selector::parse("div.xrnccd").unwrap();
        let title_sel = Selector::parse("article h3").unwrap();
        let a_sel = Selector::parse("article a[href]").unwrap();
        let time_sel = Selector::parse("article time").unwrap();
        let source_sel = Selector::parse("article a[data-n-tid]").unwrap();

        let mut results = Vec::new();
        for (i, el) in doc.select(&article_sel).enumerate() {
            let title = match el.select(&title_sel).next() {
                Some(t) => t.text().collect::<String>(),
                None => continue,
            };
            if title.is_empty() {
                continue;
            }

            // Decode the base64-encoded article URL
            let href = match el
                .select(&a_sel)
                .next()
                .and_then(|a| a.value().attr("href"))
            {
                Some(h) => match decode_google_news_url(h) {
                    Some(decoded) => decoded,
                    None => continue,
                },
                None => continue,
            };

            let pub_date = el
                .select(&time_sel)
                .next()
                .map(|t| t.text().collect::<String>())
                .unwrap_or_default();

            let source = el
                .select(&source_sel)
                .next()
                .map(|s| s.text().collect::<String>())
                .unwrap_or_default();

            let content = [source, pub_date]
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join(" / ");

            results.push(SearchResult {
                title,
                url: href,
                content,
                engine: "Google News".to_string(),
                engine_rank: (i + 1) as u32,
                    score: 0.0,
                    thumbnail: None,
                    published_date: None,
                    category: String::new(),
                    metadata: serde_json::Value::Null,
                });
        }
        Ok(results)
    }
}
