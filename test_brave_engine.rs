// Test Brave engine implementation separately
use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = "manfromexistence github";
    
    let url = format!(
        "https://search.brave.com/search?q={}&offset=0&spellcheck=1&source=web&safesearch=off",
        urlencoding::encode(query),
    );
    
    println!("Testing Brave with URL:");
    println!("{}\n", url);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en,en-US;q=0.5")
        .send()
        .await?;
    
    println!("Response status: {}", resp.status());
    println!("Response URL: {}\n", resp.url());
    
    let body = resp.text().await?;
    println!("Response body length: {} bytes\n", body.len());
    
    // Save response for inspection
    std::fs::write("brave_response.html", &body)?;
    println!("Saved response to brave_response.html\n");
    
    // Parse results
    let document = Html::parse_document(&body);
    let result_sel = Selector::parse("div.snippet[data-type='web']").unwrap();
    
    let results_count = document.select(&result_sel).count();
    println!("Found {} results with selector 'div.snippet[data-type=\"web\"]'\n", results_count);
    
    if results_count == 0 {
        // Try alternative selectors
        let alt_selectors = vec!["div.snippet", "div[data-type='web']", ".fdb", "div.result"];
        for selector_str in alt_selectors {
            if let Ok(sel) = Selector::parse(selector_str) {
                let count = document.select(&sel).count();
                println!("Alternative selector '{}': {} results", selector_str, count);
            }
        }
    } else {
        // Show first 3 results
        let title_sel = Selector::parse("a.result-header span.snippet-title").unwrap();
        let link_sel = Selector::parse("a.result-header").unwrap();
        
        for (i, element) in document.select(&result_sel).take(3).enumerate() {
            if let Some(link_el) = element.select(&link_sel).next() {
                if let Some(href) = link_el.value().attr("href") {
                    if let Some(title_el) = element.select(&title_sel).next() {
                        let title: String = title_el.text().collect();
                        println!("Result {}: {}", i + 1, title.trim());
                        println!("  URL: {}\n", href);
                    }
                }
            }
        }
    }
    
    Ok(())
}
