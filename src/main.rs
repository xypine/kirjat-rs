pub mod structs;
pub mod scrapers;
pub mod utils;

use anyhow::{Result, Context};
use moka::sync::Cache as GenericCache;
use structs::kirja::Kirja;

pub type Cache = GenericCache<String, String>;

pub fn get_page_html(url: &String, cache: &Option<&mut Cache>) -> Result<String> {
    match cache {
        Some(cache) => {
            match cache.get(url) {
                Some(data) => return Ok(data),
                None => {},
            }
        },
        None => {},
    }
    let response = reqwest::blocking::get(
        url
    )?;

    let text = response.text()?;

    if let Some(cache) = cache {
        cache.insert(url.to_string(), text.clone());
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
pub fn search_book(name: String, selected_scraper: scrapers::Scrapers, cache: Option<&mut Cache>) -> Result<Vec<Kirja>> {
    let scraper = scrapers::get_instance(selected_scraper);

    println!("Downloading page...");
    let url = scraper.get_page_url(&name);
    let html = get_page_html(&url, &cache).context("Failed to get page html")?;

    println!("Parsing html...");
    let document = parse_html(&html);

    println!("Extracting data...");
    let items = scraper.parse_document(document, &name, &cache)?;

    println!("{:#?}", items);
    Ok(items)
}

fn main() {
    let mut cache = Cache::new(10_000);
    search_book("bios 2".to_string(), scrapers::Scrapers::Sanomapro, Some(&mut cache)).unwrap();
    search_book("bios 2".to_string(), scrapers::Scrapers::Sanomapro, Some(&mut cache)).unwrap();
}