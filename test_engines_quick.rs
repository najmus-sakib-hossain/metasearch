#!/usr/bin/env rust-script
//! Quick engine test - checks which engines are configured and can be instantiated
//! 
//! ```cargo
//! [dependencies]
//! metasearch-engine = { path = "crates/metasearch-engine" }
//! metasearch-core = { path = "crates/metasearch-core" }
//! reqwest = "0.12"
//! tokio = { version = "1", features = ["full"] }
//! ```

use metasearch_core::query::SearchQuery;
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (compatible; metasearch-test/0.1)")
        .build()
        .expect("Failed to create HTTP client");

    let registry = EngineRegistry::with_defaults(client);
    let query = SearchQuery::new("test");

    let mut engine_names = registry.engine_names();
    engine_names.sort();
    let total = engine_names.len();

    println!("\n{}", "=".repeat(80));
    println!("  QUICK ENGINE TEST - {} engines registered", total);
    println!("{}", "=".repeat(80));
    println!();

    let mut working = 0;
    let mut empty = 0;
    let mut failed = 0;
    let mut timeout = 0;
    let mut disabled = 0;

    for (i, name) in engine_names.iter().enumerate() {
        let engine = match registry.get(name) {
            Some(e) => e.clone(),
            None => {
                println!("  [{:>3}/{}] {:<35} ❌ NOT IN REGISTRY", i + 1, total, name);
                failed += 1;
                continue;
            }
        };
        
        let meta = engine.metadata();
        
        if !meta.enabled {
            println!("  [{:>3}/{}] {:<35} ⚪ DISABLED", i + 1, total, meta.display_name);
            disabled += 1;
            continue;
        }

        print!("  [{:>3}/{}] {:<35} ", i + 1, total, meta.display_name);
        
        match tokio::time::timeout(Duration::from_secs(8), engine.search(&query)).await {
            Ok(Ok(results)) if !results.is_empty() => {
                println!("✅ {} results", results.len());
                working += 1;
            }
            Ok(Ok(_)) => {
                println!("⚠️  0 results");
                empty += 1;
            }
            Ok(Err(e)) => {
                let msg = format!("{e}");
                let short = if msg.len() > 45 { &msg[..45] } else { &msg };
                println!("❌ {}", short);
                failed += 1;
            }
            Err(_) => {
                println!("⏱️  timeout");
                timeout += 1;
            }
        }
    }

    println!();
    println!("{}", "=".repeat(80));
    println!("  SUMMARY:");
    println!("    ✅ Working:  {} ({:.1}%)", working, working as f64 / total as f64 * 100.0);
    println!("    ⚠️  Empty:    {} ({:.1}%)", empty, empty as f64 / total as f64 * 100.0);
    println!("    ❌ Failed:   {} ({:.1}%)", failed, failed as f64 / total as f64 * 100.0);
    println!("    ⏱️  Timeout:  {} ({:.1}%)", timeout, timeout as f64 / total as f64 * 100.0);
    println!("    ⚪ Disabled: {} ({:.1}%)", disabled, disabled as f64 / total as f64 * 100.0);
    println!("    📊 Total:    {}", total);
    println!("{}", "=".repeat(80));
    println!();
}
