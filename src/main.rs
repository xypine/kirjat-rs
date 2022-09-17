pub mod structs;
pub mod scrapers;
pub mod utils;

use anyhow::{Result, Context};

use crate::scrapers::Scraper;

pub fn get_page_html(url: &String) -> Result<String> {
    let response = reqwest::blocking::get(
        url
    )?;

    response.text().context("Failed to get page text content")
}

pub fn parse_html(raw: &str) -> scraper::Html {
    scraper::Html::parse_document(raw)
}

/// The main method you should be using
pub fn search_book(name: String, selected_scraper: scrapers::Scrapers) -> Result<()> {
    let scraper = scrapers::get_instance(scrapers::Scrapers::Jamera);

    println!("Downloading page...");
    let url = scraper.get_page_url(&name);
    let html = get_page_html(&url).context("Failed to get page html")?;

    println!("Parsing html...");
    let document = parse_html(&html);

    println!("Extracting data...");
    let items = scraper.parse_document(document)?;

    println!("{:#?}", items);
    Ok(())
}

fn main() {
    let e = structs::currency::Currency::from_euros_and_cents(200, 5);
    println!("{}", e);
    search_book("bios 2".to_string(), scrapers::Scrapers::Jamera).unwrap();
}