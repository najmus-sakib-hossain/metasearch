// Test Bing engine implementation separately
use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = "manfromexistence github";
    
    let url = format!(
        "https://www.bing.com/search?q={}&pq={}&first=1",
        urlencoding::encode(query),
        urlencoding::encode(query),
    );
    
    println!("Testing Bing with URL:");
    println!("{}\n", url);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en,en-US;q=0.9")
        .header("Cookie", "SRCHHPGUSR=ADLT=OFF; _EDGE_CD=m=US&u=en; _EDGE_S=mkt=US&ui=en")
        .send()
        .await?;
    
    println!("Response status: {}", resp.status());
    println!("Response URL: {}\n", resp.url());
    
    let body = resp.text().await?;
    println!("Response body length: {} bytes\n", body.len());
    
    // Save response for inspection
    std::fs::write("bing_response.html", &body)?;
    println!("Saved response to bing_response.html\n");
    
    // Parse results
    let document = Html::parse_document(&body);
    let result_selector = Selector::parse("li.b_algo").unwrap();
    
    let results_count = document.select(&result_selector).count();
    println!("Found {} results with selector 'li.b_algo'\n", results_count);
    
    if results_count == 0 {
        // Try alternative selectors
        let alt_selectors = vec!["ol#b_results li", "div.b_algo", ".b_algo"];
        for selector_str in alt_selectors {
            if let Ok(sel) = Selector::parse(selector_str) {
                let count = document.select(&sel).count();
                println!("Alternative selector '{}': {} results", selector_str, count);
            }
        }
    } else {
        // Show first 3 results
        let title_selector = Selector::parse("h2 a").unwrap();
        
        for (i, element) in document.select(&result_selector).take(3).enumerate() {
            if let Some(title_el) = element.select(&title_selector).next() {
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
