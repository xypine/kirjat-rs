use crate::{structs::kirja::Kirja, Cache};

pub mod jamera;
pub mod sanomapro;
pub mod otava;

pub trait Source {
    fn get_store_name(&self) -> &'static str;
    fn get_store_url(&self) -> &'static str;

    fn get_page_url(&self, book_name: &String) -> String;
    fn parse_document(&self, document: scraper::Html, book_name: &String, cache: &Option<&mut Cache>) -> anyhow::Result<Vec<Kirja>>;
}

#[derive(Debug, Clone, Copy)]
pub enum Sources {
    Jamera,
    Sanomapro,
    Otava
}

pub const AVAILABLE_SOURCES: [Sources; 3] = [
    Sources::Jamera,
    Sources::Sanomapro,
    Sources::Otava
];

pub fn get_instance(selection: Sources) -> Box<dyn Source> {
    match selection {
        Sources::Jamera => Box::new(jamera::Jamera::new()),
        Sources::Sanomapro => Box::new(sanomapro::Sanomapro::new()),
        Sources::Otava => Box::new(otava::Otava::new())
    }
}