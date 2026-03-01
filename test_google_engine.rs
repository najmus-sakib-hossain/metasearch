// Test Google engine implementation separately
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, distributions::Alphanumeric};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let query = "manfromexistence github";
    
    // Generate arc_id like our implementation
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let random_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(23)
        .map(char::from)
        .collect();
    let arc_id = format!("arc_id:srp_{}_100,use_ac:true,_fmt:prog", random_id);
    
    let url = format!(
        "https://www.google.com/search?q={}&start=0&hl=en&lr=lang_en&ie=utf8&oe=utf8&filter=0&safe=off&asearch=arc&async={}",
        urlencoding::encode(query),
        urlencoding::encode(&arc_id),
    );
    
    println!("Testing Google with URL:");
    println!("{}\n", url);
    
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "*/*")
        .header("Accept-Language", "en,en-US;q=0.9")
        .header("Referer", "https://www.google.com/")
        .header("Cookie", "CONSENT=YES+")
        .send()
        .await?;
    
    println!("Response status: {}", resp.status());
    println!("Response URL: {}\n", resp.url());
    
    let body = resp.text().await?;
    println!("Response body length: {} bytes\n", body.len());
    
    // Save response for inspection
    std::fs::write("google_response.html", &body)?;
    println!("Saved response to google_response.html\n");
    
    // Parse results
    let document = Html::parse_document(&body);
    let result_selector = Selector::parse("div.MjjYud").unwrap();
    
    let results_count = document.select(&result_selector).count();
    println!("Found {} results with selector 'div.MjjYud'\n", results_count);
    
    if results_count == 0 {
        // Try alternative selectors
        let alt_selectors = vec!["div.g", "div.tF2Cxc", "div[data-sokoban-container]"];
        for selector_str in alt_selectors {
            if let Ok(sel) = Selector::parse(selector_str) {
                let count = document.select(&sel).count();
                println!("Alternative selector '{}': {} results", selector_str, count);
            }
        }
    } else {
        // Show first 3 results
        let title_sel = Selector::parse("div[role='link'], h3").unwrap();
        let link_sel = Selector::parse("a[href]").unwrap();
        
        for (i, element) in document.select(&result_selector).take(3).enumerate() {
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
