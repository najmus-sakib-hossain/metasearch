//! Goodreads engine — search books via HTML scraping.
//! Translated from SearXNG `searx/engines/goodreads.py`.

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

pub struct Goodreads {
    metadata: EngineMetadata,
    client: Client,
}

impl Goodreads {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "goodreads".to_string(),
                display_name: "Goodreads".to_string(),
                homepage: "https://www.goodreads.com".to_string(),
                categories: vec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 0.8,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Goodreads {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page;
        let url = format!(
            "https://www.goodreads.com/search?q={}&page={}",
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
        let row_sel = Selector::parse("table tr").unwrap();
        let title_sel = Selector::parse("a.bookTitle").unwrap();
        let author_sel = Selector::parse("a.authorName").unwrap();
        let thumb_sel = Selector::parse("img.bookCover").unwrap();
        let info_sel = Selector::parse("span.uitext").unwrap();

        let mut results = Vec::new();

        for (i, row) in document.select(&row_sel).enumerate() {
            let title_el = match row.select(&title_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title = title_el.text().collect::<String>().trim().to_string();
            let href = title_el.value().attr("href").unwrap_or("");
            let book_url = format!("https://www.goodreads.com{}", href);

            let author = row
                .select(&author_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let thumbnail = row
                .select(&thumb_sel)
                .next()
                .and_then(|el| el.value().attr("src"))
                .map(|s| s.to_string());

            let info = row
                .select(&info_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let snippet = format!("by {} — {}", author.trim(), info.trim());

            let mut result = SearchResult::new(title, book_url, snippet, "goodreads".to_string());
            result.engine_rank = (i + 1) as u32;
            result.category = SearchCategory::General.to_string();
            result.thumbnail = thumbnail;
            results.push(result);
        }

        Ok(results)
    }
}
