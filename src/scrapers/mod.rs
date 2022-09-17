use crate::structs::kirja::Kirja;

pub mod jamera;

pub trait Scraper {
    fn get_store_name() -> &'static str;
    fn get_store_url() -> &'static str;

    fn get_page_url(&self, book_name: &String) -> String;
    fn parse_document(&self, document: scraper::Html) -> anyhow::Result<Vec<Kirja>>;
}

pub enum Scrapers {
    Jamera
}

pub fn get_instance(selection: Scrapers) -> impl Scraper {
    match selection {
        Scrapers::Jamera => jamera::Jamera::new(),
    }
}