//! Search routes — the main web UI endpoints.

use std::sync::Arc;

use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
    pub categories: Option<String>,
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

async fn index() -> Html<String> {
    // TODO: Render Tera template for homepage
    Html(
        "<h1>🔍 Metasearch</h1><p>A privacy-respecting metasearch engine, built in Rust.</p>"
            .to_string(),
    )
}

async fn search(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> Html<String> {
    let query = params.q.unwrap_or_default();
    // TODO: Execute search via engine registry, cache, rank results, render template
    Html(format!(
        "<h1>Results for: {}</h1><p>Coming soon...</p>",
        query
    ))
}
