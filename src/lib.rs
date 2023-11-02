#![feature(let_chains)]
pub mod features;
pub mod sources;
pub mod structs;

use anyhow::{Context, Result};
use moka::future::Cache as GenericCache;
use reqwest::header::HeaderMap;
use sources::RequestDetails;
use structs::response::{Response, ResponseError};

pub type Cache = GenericCache<String, String>;

/// Download a page and cache it, or return it from cache
pub async fn get_page_plaintext(
    url: &String,
    headers: Option<HeaderMap>,
    cache: &Option<&mut Cache>,
) -> Result<String> {
    match cache {
        Some(cache) => match cache.get(url) {
            Some(data) => {
                return Ok(data);
            }
            None => {}
        },
        None => {}
    }
    let client = reqwest::Client::new();
    let mut request = client.get(url);
    if let Some(headers) = headers {
        request = request.headers(headers);
    }
    let response = request.send().await?;

    let text = response.text().await?;

    if let Some(cache) = cache {
        cache.insert(url.to_string(), text.clone()).await;
    }

    Ok(text)
}

/// Parse string containing html
pub fn parse_html(raw: &str) -> scraper::Html {
    scraper::Html::parse_document(raw)
}

/// Parse string containing json
pub fn parse_json(raw: &str) -> Result<serde_json::Value> {
    let a = serde_json::from_str(raw).context("")?;
    return Ok(a);
}

/// Search a book from a specific source
pub async fn search_book(
    name: &String,
    scraper: &Box<dyn sources::Source>,
    cache: &Option<&mut Cache>,
) -> Response {
    // println!("Downloading page...");
    let RequestDetails { url, headers } = scraper.get_request_details(&name).await;
    let raw_result = get_page_plaintext(&url, headers, &cache).await;

    match raw_result {
        Ok(plaintext) => {
            let items_result = scraper.parse_document(plaintext, &name, cache).await;
            match items_result {
                Ok(items) => Ok(items),
                Err(error) => Err(error),
            }
        }
        Err(error) => {
            return Err(ResponseError::NetworkError(error.to_string()));
        }
    }
}

/// Search a book from all sources included in the library
pub async fn search_book_from_all_sources(
    name: &String,
    cache: &Option<&mut Cache>,
) -> Vec<Response> {
    let handles = sources::AVAILABLE_SOURCES
        .iter()
        .map(|scraper| async {
            let scraper_instance = sources::get_instance(*scraper);
            let items = search_book(name, &scraper_instance, cache).await?;
            tracing::info!("{} done!", scraper_instance.get_store_name());
            Ok(items)
        })
        .collect::<Vec<_>>();
    futures::future::join_all(handles).await
}
