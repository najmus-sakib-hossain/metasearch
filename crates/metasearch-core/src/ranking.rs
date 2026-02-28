//! Result ranking, deduplication, and aggregation logic.

use std::collections::HashMap;
use url::Url;

use crate::result::{SearchResponse, SearchResult};

/// Aggregates results from multiple engines, deduplicates, and ranks them.
pub struct ResultAggregator {
    /// Weight multiplier per engine.
    pub engine_weights: HashMap<String, f64>,
}

impl ResultAggregator {
    pub fn new(engine_weights: HashMap<String, f64>) -> Self {
        Self { engine_weights }
    }

    /// Aggregate raw results into a final ranked response.
    pub fn aggregate(
        &self,
        query: &str,
        all_results: Vec<(String, Vec<SearchResult>)>,
        search_time_ms: u64,
    ) -> SearchResponse {
        let mut engines_used = Vec::new();
        let mut url_map: HashMap<String, SearchResult> = HashMap::new();

        for (engine_name, results) in &all_results {
            engines_used.push(engine_name.clone());

            for result in results {
                let normalized_url = Self::normalize_url(&result.url);
                let weight = self
                    .engine_weights
                    .get(&result.engine)
                    .copied()
                    .unwrap_or(1.0);

                let score = weight * (1.0 / (result.engine_rank as f64 + 1.0));

                url_map
                    .entry(normalized_url)
                    .and_modify(|existing| {
                        existing.score += score;
                    })
                    .or_insert_with(|| {
                        let mut r = result.clone();
                        r.score = score;
                        r
                    });
            }
        }

        let mut results: Vec<SearchResult> = url_map.into_values().collect();
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let number_of_results = results.len();

        SearchResponse {
            query: query.to_string(),
            results,
            number_of_results,
            engines_used,
            engines_failed: Vec::new(),
            search_time_ms,
        }
    }

    /// Normalize a URL for deduplication.
    fn normalize_url(raw: &str) -> String {
        match Url::parse(raw) {
            Ok(mut url) => {
                url.set_fragment(None);
                let _ = url.set_scheme("https");
                let path = url.path().trim_end_matches('/');
                format!(
                    "{}://{}{}",
                    url.scheme(),
                    url.host_str().unwrap_or(""),
                    path
                )
            }
            Err(_) => raw.to_lowercase(),
        }
    }
}
