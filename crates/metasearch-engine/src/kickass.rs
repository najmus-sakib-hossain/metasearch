//! Kickass Torrents engine — search torrents on KickassTorrents.
//! Translated from SearXNG `searx/engines/kickass.py`.

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use metasearch_core::{
    engine::{SearchEngine, EngineMetadata},
    result::SearchResult,
    query::SearchQuery,
    category::SearchCategory,
    error::{MetasearchError, Result},
};

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
                homepage: "https://kickassanime.am".to_string(),
                categories: vec![SearchCategory::Files],
                enabled: true,
                timeout_ms: 3000,
                weight: 0.6,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for Kickass {
    fn metadata(&self) -> &EngineMetadata {
        &self.metadata
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

        let resp = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let html_text = resp.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;

        let document = Html::parse_document(&html_text);
        let row_sel = Selector::parse("table.data tr").unwrap();
        let link_sel = Selector::parse("a.cellMainLink").unwrap();
        let desc_sel = Selector::parse("span.font11px").unwrap();
        let seed_sel = Selector::parse("td.green").unwrap();
        let leech_sel = Selector::parse("td.red").unwrap();
        let size_sel = Selector::parse("td.nobr").unwrap();

        let mut results = Vec::new();

        for (i, row) in document.select(&row_sel).skip(1).enumerate() {
            let link_el = match row.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or_default();
            let title: String = link_el.text().collect();
            let page_url = format!("{}{}", base, href);

            let content: String = row.select(&desc_sel).next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let seed: String = row.select(&seed_sel).next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let leech: String = row.select(&leech_sel).next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let filesize: String = row.select(&size_sel).next()
                .map(|el| el.text().collect())
                .unwrap_or_default();

            let snippet = format!(
                "Size: {} | Seeds: {} | Leeches: {} | {}",
                filesize.trim(), seed.trim(), leech.trim(), content.trim()
            );

            let mut result = SearchResult::new(
                title.trim().to_string(),
                page_url,
                snippet,
                "kickass".to_string(),
            );
            result.engine_rank = (i + 1) as u32;
            result.category = "files".to_string();
            results.push(result);
        }

        Ok(results)
    }
}
