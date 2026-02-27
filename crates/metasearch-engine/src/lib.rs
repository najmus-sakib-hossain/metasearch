//! # metasearch-engine
//!
//! Concrete search engine implementations.
//! Each engine scrapes or queries a search provider
//! and returns normalized `SearchResult` items.

pub mod google;
pub mod duckduckgo;
pub mod brave;
pub mod wikipedia;
pub mod registry;

pub use registry::EngineRegistry;
