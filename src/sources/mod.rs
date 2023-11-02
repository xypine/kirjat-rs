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
pub enum BuiltInSource {
    /// Redirects to Suomalainen, since Jamera doesn't exist anymore
    Jamera,
    Sanomapro,
    Otava,
    Suomalainen,
}
pub const AVAILABLE_SOURCES: [BuiltInSource; 3] = [
    BuiltInSource::Suomalainen,
    BuiltInSource::Sanomapro,
    BuiltInSource::Otava,
];

/// Get an instance of a source enum
pub fn get_instance(selection: BuiltInSource) -> Box<(dyn Source + 'static)> {
    match selection {
        BuiltInSource::Jamera => Box::new(suomalainen::Suomalainen::new()),
        BuiltInSource::Suomalainen => Box::new(suomalainen::Suomalainen::new()),
        BuiltInSource::Sanomapro => Box::new(sanomapro::Sanomapro::new()),
        BuiltInSource::Otava => Box::new(otava::Otava::new()),
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
