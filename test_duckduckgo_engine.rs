// Test DuckDuckGo engine implementation separately
use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = "manfromexistence github";
    
    let body = format!("q={}", urlencoding::encode(query));
    
    println!("Testing DuckDuckGo HTML endpoint");
    println!("POST to: https://html.duckduckgo.com/html/");
    println!("Body: {}\n", body);
    
    let resp = client
        .post("https://html.duckduckgo.com/html/")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Referer", "https://html.duckduckgo.com/")
        .header("Origin", "https://html.duckduckgo.com")
        .header("Sec-Fetch-Mode", "navigate")
        .body(body)
        .send()
        .await?;
    
    println!("Response status: {}", resp.status());
    println!("Response URL: {}\n", resp.url());
    
    let html_text = resp.text().await?;
    println!("Response body length: {} bytes\n", html_text.len());
    
    // Save response for inspection
    std::fs::write("duckduckgo_response.html", &html_text)?;
    println!("Saved response to duckduckgo_response.html\n");
    
    // Parse results
    let document = Html::parse_document(&html_text);
    let result_sel = Selector::parse("div.web-result").unwrap();
    
    let results_count = document.select(&result_sel).count();
    println!("Found {} results with selector 'div.web-result'\n", results_count);
    
    if results_count == 0 {
        // Try alternative selectors
        let alt_selectors = vec!["div.result", "div.links_main", ".result__body"];
        for selector_str in alt_selectors {
            if let Ok(sel) = Selector::parse(selector_str) {
                let count = document.select(&sel).count();
                println!("Alternative selector '{}': {} results", selector_str, count);
            }
        }
    } else {
        // Show first 3 results
        let title_sel = Selector::parse("a.result__a").unwrap();
        
        for (i, item) in document.select(&result_sel).take(3).enumerate() {
            if let Some(title_el) = item.select(&title_sel).next() {
                let title: String = title_el.text().collect();
                if let Some(href) = title_el.value().attr("href") {
                    println!("Result {}: {}", i + 1, title.trim());
                    println!("  URL: {}\n", href);
                }
            }
        }
    }
    
    Ok(())
}
