//! Imgur engine — search images on Imgur.
//! Translated from SearXNG `searx/engines/imgur.py`.

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

pub struct Imgur {
    client: Client,
}

impl Imgur {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchEngine for Imgur {
    fn metadata(&self) -> EngineMetadata {
        EngineMetadata {
            name: "imgur".to_string(),
            display_name: "Imgur".to_string(),
            categories: vec![SearchCategory::Images],
            enabled: true,
            weight: 0.7,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, MetasearchError> {
        let page = query.page.unwrap_or(1);
        let page_index = (page as u32).saturating_sub(1);

        let url = format!(
            "https://imgur.com/search/score/all?q={}&qs=thumbs&p={}",
            urlencoding::encode(&query.query),
            page_index,
        );

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let card_sel = Selector::parse("div.cards div.post, div.post").unwrap();
        let link_sel = Selector::parse("a").unwrap();
        let img_sel = Selector::parse("a img").unwrap();

        let mut results = Vec::new();

        for (i, card) in document.select(&card_sel).enumerate() {
            let link = card.select(&link_sel).next()
                .and_then(|a| a.value().attr("href"));
            let img = card.select(&img_sel).next();

            let (thumbnail_src, title) = match img {
                Some(el) => (
                    el.value().attr("src").unwrap_or_default().to_string(),
                    el.value().attr("alt").unwrap_or_default().to_string(),
                ),
                None => continue,
            };

            if thumbnail_src.len() < 25 {
                continue;
            }

            let img_src = thumbnail_src.replace("b.", ".");
            let page_url = match link {
                Some(href) => format!("https://imgur.com{}", href),
                None => continue,
            };

            let mut result = SearchResult::new(
                title,
                page_url,
                String::new(),
                "imgur".to_string(),
            );
            result.engine_rank = Some(i + 1);
            result.category = Some(SearchCategory::Images);
            result.thumbnail = Some(thumbnail_src);
            result.image_src = Some(img_src);
            results.push(result);
        }

        Ok(results)
    }
}
