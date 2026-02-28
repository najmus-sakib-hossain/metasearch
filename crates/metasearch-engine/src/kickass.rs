//! Kickass Torrents — torrent search on KickassTorrents.
//!
//! Scrapes HTML search results from kickasstorrents.to.
//!
//! Reference: <https://kickasstorrents.to>

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

pub struct Kickass {
    metadata: EngineMetadata,
    client: Client,
}

impl Kickass {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "kickass".to_string(),
                display_name: "Kickass Torrents".to_string(),
                homepage: "https://kickasstorrents.to".to_string(),
                categories: vec![SearchCategory::Files],
                enabled: true,
                timeout_ms: 8000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Kickass {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page;
        let base = "https://kickasstorrents.to";

        let url = format!(
            "{}/usearch/{}/{}/",
            base,
            urlencoding::encode(&query.query),
            page,
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
        let row_sel = Selector::parse("table.data tr")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel = Selector::parse("a.cellMainLink")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let seed_sel = Selector::parse("td.green")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let leech_sel =
            Selector::parse("td.red").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let size_sel = Selector::parse("td.nobr")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (rank, row) in document.select(&row_sel).skip(1).enumerate() {
            let link_el = match row.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or_default();
            let title: String = link_el.text().collect();
            let page_url = format!("{}{}", base, href);

            let seed: String = row
                .select(&seed_sel)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let leech: String = row
                .select(&leech_sel)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let filesize: String = row
                .select(&size_sel)
                .next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let snippet = format!(
                "Size: {} | Seeds: {} | Leeches: {}",
                filesize.trim(),
                seed.trim(),
                leech.trim()
            );

            let mut result = SearchResult::new(
                title.trim().to_string(),
                page_url,
                snippet,
                self.metadata.name.clone(),
            );
            result.engine_rank = (rank + 1) as u32;
            results.push(result);
        }

        Ok(results)
    }
}
