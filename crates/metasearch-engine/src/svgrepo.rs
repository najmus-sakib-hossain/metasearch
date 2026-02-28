//! SVG Repo — SVG icon search engine.
//!
//! Scrapes HTML results from svgrepo.com.

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

pub struct SvgRepo {
    metadata: EngineMetadata,
    client: Client,
}

impl SvgRepo {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "svgrepo".to_string(),
                display_name: "SVG Repo".to_string(),
                homepage: "https://www.svgrepo.com".to_string(),
                categories: vec![SearchCategory::Images],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for SvgRepo {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let page = query.page;
        let url = format!(
            "https://www.svgrepo.com/vectors/{}/{}/",
            urlencoding::encode(&query.query),
            page
        );

        let body = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?
            .text()
            .await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;

        let document = Html::parse_document(&body);
        let item_sel = Selector::parse("div.style_nodeListing__7Nmro > div")
            .map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let link_sel =
            Selector::parse("a").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;
        let img_sel =
            Selector::parse("img").map_err(|e| MetasearchError::ParseError(format!("{e:?}")))?;

        let mut results = Vec::new();

        for (i, item) in document.select(&item_sel).enumerate() {
            let link_el = match item.select(&link_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let href = link_el.value().attr("href").unwrap_or("");
            let title = link_el
                .value()
                .attr("title")
                .unwrap_or("")
                .replace(" SVG File", "")
                .replace("Show ", "");
            let img_src = item
                .select(&img_sel)
                .next()
                .and_then(|e| e.value().attr("src"))
                .unwrap_or("")
                .to_string();

            let page_url = format!("https://www.svgrepo.com{}", href);
            let mut r =
                SearchResult::new(title, page_url, String::new(), self.metadata.name.clone());
            r.engine_rank = (i + 1) as u32;
            r.thumbnail = Some(img_src);
            results.push(r);
        }

        Ok(results)
    }
}
