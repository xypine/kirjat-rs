pub mod features;
pub mod sources;
pub mod structs;
pub mod utils;

use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use moka::sync::Cache as GenericCache;
use structs::kirja::Kirja;

pub type Cache = GenericCache<String, (Instant, String)>;
pub const MAX_CACHE_DURATION: Duration = Duration::from_secs(86_400); // 24 hours

pub fn get_page_html(url: &String, cache: &Option<&mut Cache>) -> Result<String> {
    match cache {
        Some(cache) => match cache.get(url) {
            Some((time, data)) => {
                if time.elapsed() > MAX_CACHE_DURATION {
                    cache.invalidate(url);
                } else {
                    return Ok(data);
                }
            }
            None => {}
        },
        None => {}
    }
    let response = reqwest::blocking::get(url)?;

    let text = response.text()?;

    if let Some(cache) = cache {
        cache.insert(url.to_string(), (Instant::now(), text.clone()));
    }

    Ok(text)
}

pub fn parse_html(raw: &str) -> scraper::Html {
    scraper::Html::parse_document(raw)
}

pub fn parse_json(raw: &str) -> Result<serde_json::Value> {
    let a = serde_json::from_str(raw).context("")?;
    return Ok(a);
}

/// The main method you should be using
pub fn search_book(
    name: &String,
    selected_scraper: sources::Sources,
    cache: &Option<&mut Cache>,
) -> Result<Vec<Kirja>> {
    let scraper = sources::get_instance(selected_scraper);

    // println!("Downloading page...");
    let url = scraper.get_page_url(&name);
    let html = get_page_html(&url, &cache).context("Failed to get page html")?;

    // println!("Parsing html...");
    let document = parse_html(&html);

    // println!("Extracting data...");
    let items = scraper.parse_document(document, &name, cache)?;
    Ok(items)
}

pub fn search_book_from_all_sources(
    name: &String,
    cache: &Option<&mut Cache>,
) -> Result<Vec<Kirja>> {
    let mut out = vec![];
    for scraper in sources::AVAILABLE_SOURCES {
        let mut items = search_book(name, scraper, cache)?;
        out.append(&mut items);
    }

    Ok(out)
}

fn main() {
    #[cfg(feature = "tui")]
    features::tui::start_tui();
    #[cfg(not(feature = "tui"))]
    println!("The crate wasn't built with the \"tui\"-option, so no ui is available.");
}
