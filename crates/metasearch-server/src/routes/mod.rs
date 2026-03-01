//! Route definitions.

pub mod api;
pub mod health;
pub mod search;

use crate::state::AppState;
use axum::Router;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn search_routes() -> Router<Arc<AppState>> {
    search::routes()
}

pub fn api_routes() -> Router<Arc<AppState>> {
    api::routes()
}

pub fn static_routes() -> Router<Arc<AppState>> {
    // Serve static files from /static/* path
    // The static folder is at the workspace root
    Router::new().nest_service("/static", ServeDir::new("static"))
}

pub fn health_routes() -> Router<Arc<AppState>> {
    health::routes()
}
