//! # metasearch-server
//!
//! The Axum-based HTTP server that powers the metasearch web UI and API.

pub mod app;
pub mod routes;
pub mod middleware;
pub mod state;
pub mod cache;
pub mod templates;
