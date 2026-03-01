//! Search orchestrator — coordinates cache, engines, and result aggregation.
//!
//! Architecture:
//! 1. Check 3-layer cache (hit/coalesce/miss)
//! 2. Fan-out to all engines for the category using `FuturesUnordered`
//!    — results stream in as engines respond (fastest first)
//! 3. Aggregate with `FxHashMap` (2-3× faster than std HashMap for short keys)
//! 4. Sort with `sort_unstable_by` (faster than stable sort, no rayon needed
//!    for < 1 000 items — rayon overhead > sort cost at this scale)
//! 5. Store in cache wrapped in `Arc` (zero-copy sharing between concurrent readers)

use std::sync::Arc;
use std::time::Instant;

use futures::stream::{FuturesUnordered, StreamExt};
use metasearch_core::{
    category::SearchCategory,
    engine::SearchEngine,
    query::SearchQuery,
    result::{SearchResponse, SearchResult},
};
use metasearch_engine::EngineRegistry;
use rustc_hash::FxHashMap;

use crate::cache::{CacheLookup, SearchCache};
use crate::health::EngineHealthTracker;

/// Central coordinator for all search operations.
pub struct SearchOrchestrator {
    pub registry: Arc<EngineRegistry>,
    pub cache: SearchCache,
    pub health: Arc<EngineHealthTracker>,
    /// Max engines to fan-out to per request (prevents thundering herd).
    pub max_engines: usize,
}

impl SearchOrchestrator {
    pub fn new(
        registry: Arc<EngineRegistry>,
        cache: SearchCache,
        health: Arc<EngineHealthTracker>,
        max_engines: usize,
    ) -> Self {
        Self {
            registry,
            cache,
            health,
            max_engines,
        }
    }

    /// Execute a search, using the cache when possible.
    pub async fn search(
        &self,
        query: &SearchQuery,
        cache_key: &str,
    ) -> Arc<SearchResponse> {
        match self.cache.get_or_coalesce(cache_key).await {
            CacheLookup::Hit(r) => {
                tracing::debug!(key = cache_key, "Cache hit");
                r
            }
            CacheLookup::Coalesced(r) => {
                tracing::debug!(key = cache_key, "Coalesced with in-flight request");
                r
            }
            CacheLookup::Miss(guard) => {
                let response = Arc::new(self.execute_search(query).await);
                guard.complete(Arc::clone(&response)).await;
                response
            }
        }
    }

    /// Fan-out to all eligible engines and aggregate results.
    async fn execute_search(&self, query: &SearchQuery) -> SearchResponse {
        let start = Instant::now();

        let category = query
            .categories
            .first()
            .copied()
            .unwrap_or(SearchCategory::General);

        // Get engines for this category, skip circuit-broken ones
        let engines: Vec<Arc<dyn SearchEngine>> = self
            .registry
            .engines_for_category(&category)
            .into_iter()
            .filter(|e| {
                let meta = e.metadata();
                meta.enabled && self.health.is_healthy(meta.name.as_ref())
            })
            .take(self.max_engines)
            .collect();

        let engines_queried = engines.len();

        // ── Fan-out: FuturesUnordered polls futures as they complete ─────────
        // For pure I/O tasks (HTTP) this avoids spawning N Tokio tasks —
        // no scheduling overhead, no thread-pool cross-talk.
        let mut futures = FuturesUnordered::new();

        for engine in engines {
            let query = query.clone();
            let health = Arc::clone(&self.health);

            futures.push(async move {
                let meta = engine.metadata();
                let name = meta.name.as_ref().to_owned();
                let timeout_dur = health.timeout_for(&name, meta.timeout_ms);
                let weight = meta.weight;

                let t0 = Instant::now();
                let result = tokio::time::timeout(timeout_dur, engine.search(&query)).await;
                let latency_ms = t0.elapsed().as_millis() as u32;

                match result {
                    Ok(Ok(results)) => {
                        health.record_success(&name, latency_ms);
                        tracing::debug!(engine = %name, count = results.len(), latency_ms, "Engine responded");
                        Some((name, weight, results))
                    }
                    Ok(Err(e)) => {
                        health.record_failure(&name);
                        tracing::warn!(engine = %name, error = %e, "Engine error");
                        None
                    }
                    Err(_) => {
                        health.record_failure(&name);
                        tracing::warn!(engine = %name, timeout_ms = timeout_dur.as_millis(), "Engine timeout");
                        None
                    }
                }
            });
        }

        // ── Aggregate: stream results as they arrive ─────────────────────────
        // FxHashMap: 2-3× faster than std HashMap for short string keys.
        let mut url_map: FxHashMap<String, SearchResult> =
            FxHashMap::with_capacity_and_hasher(256, Default::default());
        let mut engines_used: Vec<String> = Vec::new();
        let mut engines_failed: Vec<String> = Vec::new();

        // Early return thresholds for sub-second response
        const MIN_ENGINES: usize = 5;  // Wait for at least 5 engines
        const MIN_RESULTS: usize = 50; // Or 50 unique results
        const MAX_WAIT_MS: u128 = 800; // But never wait more than 800ms

        while let Some(result) = futures.next().await {
            match result {
                Some((engine_name, weight, results)) => {
                    engines_used.push(engine_name.clone());
                    for mut r in results {
                        // Base score from engine rank and weight
                        let base_score = weight * (1.0 / (r.engine_rank as f64 + 1.0));
                        
                        // Relevance boost based on query term matches
                        let relevance_boost = calculate_relevance(&r, &query.query);
                        
                        // Final score combines base score with relevance
                        let score = base_score * (1.0 + relevance_boost);
                        
                        let normalized = normalize_url(&r.url);
                        url_map
                            .entry(normalized)
                            .and_modify(|existing| {
                                existing.score += score;
                            })
                            .or_insert_with(|| {
                                r.score = score;
                                r
                            });
                    }
                }
                None => {
                    // Engine failed/timed out — name already logged; record as failed
                    // (we can't get the name back here, so just count the failure)
                    engines_failed.push("unknown".to_string());
                }
            }

            // Early return: stop waiting if we have enough results
            let elapsed = start.elapsed().as_millis();
            let have_enough = engines_used.len() >= MIN_ENGINES && url_map.len() >= MIN_RESULTS;
            let timeout_reached = elapsed >= MAX_WAIT_MS;
            
            if have_enough || timeout_reached {
                if have_enough {
                    tracing::debug!(
                        engines = engines_used.len(),
                        results = url_map.len(),
                        elapsed_ms = elapsed,
                        "Early return: sufficient results"
                    );
                } else {
                    tracing::debug!(
                        engines = engines_used.len(),
                        results = url_map.len(),
                        elapsed_ms = elapsed,
                        "Early return: timeout reached"
                    );
                }
                break;
            }
        }

        // ── Sort: single-threaded is faster for < 1 000 items ────────────────
        // rayon overhead (~50 µs) >> sort cost (~5 µs) for typical result sets.
        let mut results: Vec<SearchResult> = url_map.into_values().collect();
        results.sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let number_of_results = results.len();
        let search_time_ms = start.elapsed().as_millis() as u64;

        tracing::info!(
            engines_queried,
            engines_responded = engines_used.len(),
            results = number_of_results,
            search_time_ms,
            "Search complete"
        );

        SearchResponse {
            query: query.query.clone(),
            results,
            number_of_results,
            engines_used,
            engines_failed,
            search_time_ms,
        }
    }
}

/// Calculate relevance boost based on query term matches in title, URL, and content.
/// Returns a multiplier (0.0 to 10.0) where higher = more relevant.
fn calculate_relevance(result: &SearchResult, query: &str) -> f64 {
    let query_lower = query.to_lowercase();
    let query_terms: Vec<&str> = query_lower
        .split_whitespace()
        .filter(|t| t.len() > 2) // Ignore very short terms
        .collect();
    
    if query_terms.is_empty() {
        return 0.0;
    }
    
    let title_lower = result.title.to_lowercase();
    let url_lower = result.url.to_lowercase();
    let content_lower = result.content.to_lowercase();
    
    // Penalize weather/currency/definition results for non-weather queries
    if (result.engine.contains("weather") || result.engine.contains("currency") || 
        result.engine.contains("definition")) && 
       !query_lower.contains("weather") && !query_lower.contains("currency") {
        return 0.0; // No boost for irrelevant utility results
    }
    
    let mut boost: f64 = 0.0;
    
    for term in &query_terms {
        // Exact phrase match in title = huge boost
        if title_lower.contains(&query_lower) {
            boost += 5.0;
        }
        
        // Term in title = high boost
        if title_lower.contains(term) {
            boost += 2.0;
        }
        
        // Term in URL = medium boost (especially for domain/path)
        if url_lower.contains(term) {
            boost += 1.5;
        }
        
        // Term in content = small boost
        if content_lower.contains(term) {
            boost += 0.5;
        }
    }
    
    // Bonus for matching all terms
    let all_terms_in_title = query_terms.iter().all(|t| title_lower.contains(t));
    if all_terms_in_title {
        boost += 3.0;
    }
    
    // Cap the boost to prevent extreme scores
    boost.min(10.0)
}

/// Normalize a URL for deduplication (strip fragment, normalise scheme/trailing slash).
fn normalize_url(raw: &str) -> String {
    // Fast path: avoid URL parsing overhead for most cases
    let s = raw.trim_end_matches('/');
    if let Some(no_frag) = s.split('#').next() {
        no_frag.to_ascii_lowercase()
    } else {
        s.to_ascii_lowercase()
    }
}
