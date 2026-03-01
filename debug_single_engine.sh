#!/bin/bash
# Debug a single engine to see the actual error

ENGINE_NAME="${1:-reddit}"

cat > /tmp/test_single.rs << 'EOF'
use metasearch_core::query::SearchQuery;
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let engine_name = std::env::args().nth(1).expect("Usage: test_single <engine_name>");
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("Mozilla/5.0 (compatible; metasearch-test/0.1)")
        .build()
        .unwrap();

    let registry = EngineRegistry::with_defaults(client);
    let query = SearchQuery::new("rust programming");

    let engine = registry.get(&engine_name).expect("Engine not found");
    
    println!("Testing: {}", engine.metadata().display_name);
    
    match engine.search(&query).await {
        Ok(results) => {
            println!("✅ Success! {} results", results.len());
            for (i, r) in results.iter().take(3).enumerate() {
                println!("  {}. {} - {}", i+1, r.title, r.url);
            }
        }
        Err(e) => {
            println!("❌ Error: {:?}", e);
        }
    }
}
EOF

echo "Testing engine: $ENGINE_NAME"
cargo run --quiet --manifest-path crates/metasearch-engine/Cargo.toml --example test_single "$ENGINE_NAME" 2>&1 || \
    (cd /tmp && rustc test_single.rs --edition 2021 && ./test_single "$ENGINE_NAME")
