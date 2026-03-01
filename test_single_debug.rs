use metasearch_core::query::SearchQuery;
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("Mozilla/5.0 (compatible; metasearch-test/0.1)")
        .build()
        .expect("Failed to create HTTP client");

    let registry = EngineRegistry::with_defaults(client);
    
    let engine_name = std::env::args().nth(1).unwrap_or_else(|| "duckduckgo".to_string());
    let query_text = std::env::args().nth(2).unwrap_or_else(|| "test".to_string());
    
    let engine = registry.get(&engine_name).expect("Engine not found");
    let query = SearchQuery::new(&query_text);
    
    println!("Testing {} with query '{}'", engine_name, query_text);
    
    match engine.search(&query).await {
        Ok(results) => {
            println!("✅ Success! {} results", results.len());
            for (i, r) in results.iter().take(3).enumerate() {
                println!("  {}. {} - {}", i+1, r.title, r.url);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
}
