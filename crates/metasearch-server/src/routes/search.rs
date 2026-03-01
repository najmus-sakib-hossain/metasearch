//! Search routes — the main web UI endpoints.
//!
//! Handles homepage rendering, search execution, and result display.
//! Like SearXNG, searches run across multiple engines in parallel with
//! per-engine timeouts and results grouped by category for tabbed display.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use metasearch_core::category::SearchCategory;
use metasearch_core::engine::SearchEngine;
use metasearch_core::query::SearchQuery;
use metasearch_core::result::SearchResult;
use serde::{Deserialize, Serialize};
use tera::Context;
use tokio::time::timeout;

use crate::state::AppState;

#[derive(Deserialize, Default)]
pub struct SearchParams {
    pub q: Option<String>,
    pub category: Option<String>,
    pub language: Option<String>,
    pub page: Option<u32>,
    pub safe_search: Option<u8>,
    pub time_range: Option<String>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index))
        .route("/search", get(search))
}

/// Render the homepage.
async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let context = Context::new();
    match state.templates.render("index.html", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template error: {}", e);
            Html(format!(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Metasearch</title></head>
                <body style="background:#0a0a1a;color:#fff;font-family:sans-serif;display:flex;justify-content:center;align-items:center;min-height:100vh;">
                <div style="text-align:center"><h1>🔍 Metasearch</h1><p>Template error: {}</p>
                <p>Make sure you're running from the project root directory.</p></div></body></html>"#,
                e
            ))
        }
    }
}

/// Serializable search result for templates.
#[derive(Serialize, Clone)]
struct TemplateResult {
    title: String,
    url: String,
    content: String,
    engine: String,
    thumbnail: Option<String>,
    published_date: Option<String>,
    category: String,
    metadata: serde_json::Value,
}

/// Map string category to SearchCategory enum.
fn parse_category(cat: &str) -> SearchCategory {
    match cat {
        "images" => SearchCategory::Images,
        "videos" => SearchCategory::Videos,
        "news" => SearchCategory::News,
        "science" => SearchCategory::Science,
        "music" => SearchCategory::Music,
        "files" => SearchCategory::Files,
        "it" => SearchCategory::IT,
        "social" => SearchCategory::SocialMedia,
        "maps" => SearchCategory::Maps,
        _ => SearchCategory::General,
    }
}

/// Execute a single engine search with timeout.
async fn search_engine_with_timeout(
    engine: Arc<dyn SearchEngine>,
    query: SearchQuery,
) -> (String, std::result::Result<Vec<SearchResult>, String>) {
    let name = engine.metadata().name.clone();
    let timeout_dur = Duration::from_millis(engine.metadata().timeout_ms.max(2000));

    match timeout(timeout_dur, engine.search(&query)).await {
        Ok(Ok(results)) => (name, Ok(results)),
        Ok(Err(e)) => (name.clone(), Err(format!("{}: {}", name, e))),
        Err(_) => (name.clone(), Err(format!("{}: timeout", name))),
    }
}

/// Execute search and render results.
async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> Html<String> {
    let query_text = params.q.clone().unwrap_or_default();
    let category_str = params.category.clone().unwrap_or_else(|| "general".to_string());
    let page = params.page.unwrap_or(1);
    let language = params.language.clone();

    // If no query, render homepage
    if query_text.trim().is_empty() {
        let context = Context::new();
        return match state.templates.render("index.html", &context) {
            Ok(html) => Html(html),
            Err(_) => Html("<meta http-equiv='refresh' content='0;url=/'/>".to_string()),
        };
    }

    let start_time = Instant::now();
    let active_category = parse_category(&category_str);

    // Build search query
    let search_query = SearchQuery {
        query: query_text.clone(),
        categories: vec![active_category.clone()],
        language: language.clone(),
        safe_search: params.safe_search.unwrap_or(1),
        page,
        time_range: params.time_range.clone(),
        engines: Vec::new(),
    };

    // ── Check cache first ──
    let cache_key = format!("{}:{}:{}:{}", query_text, category_str, page, language.as_deref().unwrap_or("auto"));
    if let Some(cached) = state.cache.get(&cache_key).await {
        let search_time_ms = start_time.elapsed().as_millis();
        let template_results: Vec<TemplateResult> = cached
            .results
            .iter()
            .map(result_to_template)
            .collect();

        let number_of_results = template_results.len();
        let total_pages = ((number_of_results as f64) / 10.0).ceil().max(1.0) as u32;
        let mut context = build_context(
            &query_text, &category_str, page, &template_results,
            number_of_results, search_time_ms, &cached.engines_used,
            total_pages, &language,
        );
        context.insert("from_cache", &true);

        return match state.templates.render("results.html", &context) {
            Ok(html) => Html(html),
            Err(e) => render_error(&e.to_string()),
        };
    }

    // ── Gather engines for the active category ──
    let engines = state.engine_registry.engines_for_category(&active_category);

    if engines.is_empty() {
        // Fallback: if no engines for the selected category, try General
        let fallback_engines = state.engine_registry.engines_for_category(&SearchCategory::General);
        if fallback_engines.is_empty() {
            let mut context = Context::new();
            context.insert("query", &query_text);
            context.insert("category", &category_str);
            context.insert("page", &page);
            context.insert("results", &Vec::<TemplateResult>::new());
            context.insert("number_of_results", &0usize);
            context.insert("search_time_ms", &0u128);
            context.insert("engines_used", &Vec::<String>::new());
            context.insert("total_pages", &1u32);
            context.insert("safe_search", &"Moderate");
            context.insert("language", &language.clone().unwrap_or_else(|| "Auto".to_string()));
            context.insert("from_cache", &false);
            context.insert("engines_failed", &Vec::<String>::new());

            return match state.templates.render("results.html", &context) {
                Ok(html) => Html(html),
                Err(e) => render_error(&e.to_string()),
            };
        }
    }

    // ── Execute searches in parallel with per-engine timeout ──
    let max_concurrent = state.settings.search.max_concurrent_engines.min(engines.len());
    // Process engines in batches to avoid overwhelming the system
    let mut all_results: Vec<SearchResult> = Vec::new();
    let mut engines_used: Vec<String> = Vec::new();
    let mut engines_failed: Vec<String> = Vec::new();

    // Launch all engine searches concurrently (limited by the engine set)
    let engine_futures: Vec<_> = engines
        .into_iter()
        .take(max_concurrent.max(15)) // Cap at 15 concurrent engines per category
        .map(|engine| {
            let q = search_query.clone();
            search_engine_with_timeout(engine, q)
        })
        .collect();

    let results_vec = futures::future::join_all(engine_futures).await;

    for (engine_name, result) in results_vec {
        match result {
            Ok(results) => {
                if !results.is_empty() {
                    tracing::info!(engine = %engine_name, count = results.len(), "Results received");
                    engines_used.push(engine_name);
                    all_results.extend(results);
                }
            }
            Err(e) => {
                tracing::warn!("Engine failed: {}", e);
                engines_failed.push(engine_name);
            }
        }
    }

    let search_time_ms = start_time.elapsed().as_millis();

    // ── Deduplicate by URL (keep highest-ranked) ──
    let mut seen_urls: HashMap<String, usize> = HashMap::new();
    let mut deduped_results: Vec<SearchResult> = Vec::new();
    for result in &all_results {
        let normalized = result.url.trim_end_matches('/').to_lowercase();
        if let Some(&existing_idx) = seen_urls.get(&normalized) {
            // Keep the one with higher weight — but also note the engine
            if result.score > deduped_results[existing_idx].score {
                deduped_results[existing_idx] = result.clone();
            }
        } else {
            seen_urls.insert(normalized, deduped_results.len());
            deduped_results.push(result.clone());
        }
    }

    // ── Convert to template format ──
    let template_results: Vec<TemplateResult> = deduped_results
        .iter()
        .map(result_to_template)
        .collect();

    let number_of_results = template_results.len();
    let total_pages = ((number_of_results as f64) / 10.0).ceil().max(1.0) as u32;

    // ── Cache the results ──
    let cached_response = metasearch_core::result::SearchResponse {
        query: query_text.clone(),
        results: deduped_results,
        number_of_results,
        engines_used: engines_used.clone(),
        engines_failed: engines_failed.clone(),
        search_time_ms: search_time_ms as u64,
    };
    state.cache.insert(cache_key, cached_response).await;

    // ── Build template context ──
    let mut context = build_context(
        &query_text, &category_str, page, &template_results,
        number_of_results, search_time_ms, &engines_used,
        total_pages, &language,
    );
    context.insert("from_cache", &false);
    context.insert("engines_failed", &engines_failed);

    match state.templates.render("results.html", &context) {
        Ok(html) => Html(html),
        Err(e) => render_error(&e.to_string()),
    }
}

/// Convert a SearchResult to a TemplateResult.
fn result_to_template(r: &SearchResult) -> TemplateResult {
    TemplateResult {
        title: r.title.clone(),
        url: r.url.clone(),
        content: strip_html_tags(&r.content),
        engine: r.engine.clone(),
        thumbnail: r.thumbnail.clone(),
        published_date: r.published_date.map(|d| d.format("%b %d, %Y").to_string()),
        category: r.category.clone(),
        metadata: r.metadata.clone(),
    }
}

/// Strip HTML tags from a string (Wikipedia snippets etc. contain raw HTML).
fn strip_html_tags(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    // Collapse multiple spaces
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Build the Tera context for the results template.
#[allow(clippy::too_many_arguments)]
fn build_context(
    query: &str,
    category: &str,
    page: u32,
    results: &[TemplateResult],
    number_of_results: usize,
    search_time_ms: u128,
    engines_used: &[String],
    total_pages: u32,
    language: &Option<String>,
) -> Context {
    let mut context = Context::new();
    context.insert("query", query);
    context.insert("category", category);
    context.insert("page", &page);
    context.insert("results", results);
    context.insert("number_of_results", &number_of_results);
    context.insert("search_time_ms", &(search_time_ms as u64));
    context.insert("engines_used", engines_used);
    context.insert("total_pages", &total_pages);
    context.insert("safe_search", &"Moderate");
    context.insert(
        "language",
        &language.clone().unwrap_or_else(|| "Auto".to_string()),
    );
    context
}

/// Render an error page.
fn render_error(msg: &str) -> Html<String> {
    tracing::error!("Template error: {}", msg);
    Html(format!(
        r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Error</title></head>
        <body style="background:#0a0a1a;color:#fff;font-family:sans-serif;display:flex;justify-content:center;align-items:center;min-height:100vh;">
        <div style="text-align:center"><h1>Search Error</h1><p>{}</p>
        <a href="/" style="color:#3b82f6;">Back to Home</a></div></body></html>"#,
        msg
    ))
}
