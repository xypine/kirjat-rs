//! Sources for book information

use async_trait::async_trait;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{structs::response::Response, Cache};

pub mod otava;
pub mod sanomapro;
pub mod suomalainen;

/// A shared trait for all sources, async_trait is required for async functions.
/// This library ships with a few sources, but you can implement your own as well.
#[async_trait(?Send)]
pub trait Source {
    fn get_store_name(&self) -> &'static str;
    fn get_store_url(&self) -> &'static str;

    async fn get_request_details(&self, book_name: &String) -> RequestDetails;

    async fn parse_document(
        &self,
        plaintext: String,
        book_name: &String,
        cache: &Option<&mut Cache>,
    ) -> Response;
}
pub struct RequestDetails {
    pub url: String,
    pub headers: Option<HeaderMap>,
}

/// All included sources
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Sources {
    Suomalainen,
    Sanomapro,
    Otava,
}
pub const AVAILABLE_SOURCES: [Sources; 3] =
    [Sources::Suomalainen, Sources::Sanomapro, Sources::Otava];

/// Get an instance of a source enum
pub fn get_instance(selection: Sources) -> Box<dyn Source> {
    match selection {
        Sources::Suomalainen => Box::new(suomalainen::Suomalainen::new()),
        Sources::Sanomapro => Box::new(sanomapro::Sanomapro::new()),
        Sources::Otava => Box::new(otava::Otava::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::AVAILABLE_SOURCES;

    #[test]
    fn test_all() {
        for prov in AVAILABLE_SOURCES {
            let inst = super::get_instance(prov);
        }
    }
}
