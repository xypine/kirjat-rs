use crate::{structs::kirja::Kirja, Cache};

pub mod jamera;
pub mod sanomapro;

pub trait Scraper {
    fn get_store_name(&self) -> &'static str;
    fn get_store_url(&self) -> &'static str;

    fn get_page_url(&self, book_name: &String) -> String;
    fn parse_document(&self, document: scraper::Html, book_name: &String, cache: &Option<&mut Cache>) -> anyhow::Result<Vec<Kirja>>;
}

pub enum Scrapers {
    Jamera,
    Sanomapro
}

pub fn get_instance(selection: Scrapers) -> Box<dyn Scraper> {
    match selection {
        Scrapers::Jamera => Box::new(jamera::Jamera::new()),
        Scrapers::Sanomapro => Box::new(sanomapro::Sanomapro::new())
    }
}