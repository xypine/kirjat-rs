use crate::{structs::kirja::Kirja, Cache};

pub mod jamera;
pub mod sanomapro;

pub trait Source {
    fn get_store_name(&self) -> &'static str;
    fn get_store_url(&self) -> &'static str;

    fn get_page_url(&self, book_name: &String) -> String;
    fn parse_document(&self, document: scraper::Html, book_name: &String, cache: &Option<&mut Cache>) -> anyhow::Result<Vec<Kirja>>;
}

pub enum Sources {
    Jamera,
    Sanomapro
}

pub const AVAILABLE_SOURCES: [Sources; 2] = [
    Sources::Jamera,
    Sources::Sanomapro
];

pub fn get_instance(selection: Sources) -> Box<dyn Source> {
    match selection {
        Sources::Jamera => Box::new(jamera::Jamera::new()),
        Sources::Sanomapro => Box::new(sanomapro::Sanomapro::new())
    }
}