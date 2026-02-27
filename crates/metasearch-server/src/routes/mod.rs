//! Route definitions.

pub mod search;
pub mod api;
pub mod health;

use std::sync::Arc;
use axum::Router;
use crate::state::AppState;

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
