//! # metasearch-core
//!
//! Core types, traits, and result models for the metasearch engine.
//! This crate is engine-agnostic and defines the shared contract
//! that all search engines must implement.

pub mod config;
pub mod engine;
pub mod error;
pub mod query;
pub mod result;
pub mod ranking;
pub mod category;

pub use config::Settings;
pub use engine::{SearchEngine, EngineMetadata};
pub use error::MetasearchError;
pub use query::SearchQuery;
pub use result::SearchResult;
