use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{structs::response::Response, Cache};

pub mod jamera;
pub mod otava;
pub mod sanomapro;

#[async_trait(?Send)]
pub trait Source {
    fn get_store_name(&self) -> &'static str;
    fn get_store_url(&self) -> &'static str;

    async fn get_page_url(&self, book_name: &String) -> String;
    async fn parse_document(
        &self,
        document: scraper::Html,
        book_name: &String,
        cache: &Option<&mut Cache>,
    ) -> Response;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Sources {
    Jamera,
    Sanomapro,
    Otava,
}

pub const AVAILABLE_SOURCES: [Sources; 3] = [Sources::Jamera, Sources::Sanomapro, Sources::Otava];

pub fn get_instance(selection: Sources) -> Box<dyn Source> {
    match selection {
        Sources::Jamera => Box::new(jamera::Jamera::new()),
        Sources::Sanomapro => Box::new(sanomapro::Sanomapro::new()),
        Sources::Otava => Box::new(otava::Otava::new()),
    }
}
