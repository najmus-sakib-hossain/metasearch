//! JSON API routes for programmatic access.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct ApiSearchParams {
    pub q: String,
    pub format: Option<String>,
    pub categories: Option<String>,
    pub language: Option<String>,
    pub page: Option<u32>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/search", get(api_search))
        .route("/api/v1/engines", get(api_engines))
        .route("/api/v1/config", get(api_config))
}

async fn api_search(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<ApiSearchParams>,
) -> Json<serde_json::Value> {
    // TODO: Execute search and return JSON
    Json(serde_json::json!({
        "query": params.q,
        "results": [],
        "engines_used": [],
        "search_time_ms": 0
    }))
}

async fn api_engines(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let names = state.engine_registry.engine_names();
    Json(serde_json::json!({ "engines": names }))
}

async fn api_config(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "safe_search": state.settings.search.safe_search,
        "default_language": state.settings.search.default_language,
        "image_proxy": state.settings.server.image_proxy,
    }))
}
