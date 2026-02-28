//! Route definitions.

pub mod api;
pub mod health;
pub mod search;

use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub fn search_routes() -> Router<Arc<AppState>> {
    search::routes()
}

pub fn api_routes() -> Router<Arc<AppState>> {
    api::routes()
}

pub fn static_routes() -> Router<Arc<AppState>> {
    // TODO: Serve static files (CSS, JS, images)
    Router::new()
}

pub fn health_routes() -> Router<Arc<AppState>> {
    health::routes()
}
