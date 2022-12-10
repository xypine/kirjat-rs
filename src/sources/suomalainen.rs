use super::{RequestDetails, Source};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    structs::{
        currency::Currency,
        kirja::{Condition, Kirja, Links},
        response::Response,
    },
    Cache,
};

const SEARCH_API_APP_ID: &str = "VRA8QOJXXV";
const SEARCH_API_KEY: &str = "7722fa2238d9d6a6acc37f807e90d4e7";
const SEARCH_API_URL: &str = "https://vra8qojxxv-dsn.algolia.net/1/indexes";
const SEARCH_API_INDEX: &str = "shopify_prod_products";
const PRODUCT_URL_BASE: &str = "https://www.suomalainen.com/products";

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Suomalainen {}

impl Suomalainen {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl Source for Suomalainen {
    fn get_store_name(&self) -> &'static str {
        "Suomalainen Kirjakauppa"
    }

    fn get_store_url(&self) -> &'static str {
        "https://www.suomalainen.com/"
    }

    async fn get_request_details(&self, book_name: &String) -> RequestDetails {
        let url = format!("{SEARCH_API_URL}/{SEARCH_API_INDEX}/?query={}", book_name);
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-algolia-api-key",
            HeaderValue::from_static(SEARCH_API_KEY),
        );
        headers.insert(
            "x-algolia-application-id",
            HeaderValue::from_static(SEARCH_API_APP_ID),
        );
        RequestDetails {
            url,
            headers: Some(headers),
        }
    }

    async fn parse_document(
        &self,
        plaintext: String,
        _book_name: &String,
        _cache: &Option<&mut Cache>,
    ) -> Response {
        let mut out = vec![];
        let data = crate::parse_json(&plaintext)?;

        match data {
            serde_json::Value::Object(data) => {
                if let Some(hits) = data.get("hits") {
                    match hits {
                        serde_json::Value::Array(arr) => {
                            for hit in arr {
                                match hit {
                                    serde_json::Value::Object(map) => {
                                        if 
                                            let Some(id) = map.get("id") &&
                                            let Some(handle) = map.get("handle") &&
                                            let Some(title) = map.get("title") &&
                                            let Some(image) = map.get("product_image") &&
                                            let Some(price) = map.get("price") && 
                                            let Some(product_type) = map.get("product_type")
                                        {
                                            if 
                                                let serde_json::Value::Number(id) = id &&
                                                let Some(id) = id.as_i64() &&
                                                let serde_json::Value::String(handle) = handle &&
                                                let serde_json::Value::String(title) = title &&
                                                let serde_json::Value::String(image) = image &&
                                                let serde_json::Value::Number(price) = price && 
                                                let Some(price) = price.as_f64() && 
                                                let serde_json::Value::String(product_type) = product_type
                                            {
                                                let url = format!("{PRODUCT_URL_BASE}/{handle}");
                                                out.push(Kirja {
                                                    name: title.to_string(),
                                                    id: id.to_string(),
                                                    links: Links {
                                                        buy: url.to_string(),
                                                        image: Some(image.to_string())
                                                    },
                                                    source: self.get_store_url().to_string(),
                                                    conditions: vec![
                                                        Condition {
                                                            name: product_type.to_string(),
                                                            price: Currency::from_euros(price),
                                                            ..Default::default()
                                                        }
                                                    ]
                                                })
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Ok(out)
    }
}
