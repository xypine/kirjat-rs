use anyhow::{Context, Result};
use async_trait::async_trait;
use scraper::{Selector, ElementRef};

use crate::{structs::{kirja::{Kirja, Links, Condition}, currency::Currency, response::Response}, Cache};

use super::Source;

const SEARCH_API_URL_BASE: &str = "https://api.addsearch.com/v1/search/";

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sanomapro {

}

impl Sanomapro {
    pub fn new() -> Self {
        Self {

        }
    }

    fn extract_key(&self, document: scraper::Html) -> Result<String> {
        let html = document.root_element().html();
        let key_step = html.split("?key=")
            .collect::<Vec<&str>>().get(1).context("couldn't extract search key (step 1)")?
            .split("&").collect::<Vec<&str>>();
        let key = *key_step.get(0).context("couldn't extract search key (step 2)")?;
        Ok(key.to_string())
    }

    async fn get_conditions(&self, url: &String, cache: &Option<&mut Cache>) -> Result<Vec<Condition>> {
        //println!("Fetching conditions from url {}...", url);
        let mut out = vec![];

        let html = crate::get_page_html(url, cache).await?;
        let document = crate::parse_html(&html);

        let options_selector = Selector::parse(".nested.options-list")
            .expect("Failed to construct selector");
        let options_containers = document.select(&options_selector)
            .collect::<Vec<ElementRef>>();
        let options_container = options_containers
            .first()
            .context("Failed to locate option container")?;
        
        let option_selector = Selector::parse(".field.choice")
            .expect("Failed to construct selector");
        let name_selector = Selector::parse(".product-name")
            .expect("Failed to construct selector");
        let price_selector = Selector::parse(".price")
            .expect("Failed to construct selector");
        for option in options_container.select(&option_selector) {
            let names = option.select(&name_selector).collect::<Vec<ElementRef>>();
            let prices = option.select(&price_selector).collect::<Vec<ElementRef>>();

            let name = names.first().context("Condition has no name")?
                .text().collect::<Vec<&str>>().join("");
            
            let price_str = prices.first().context("Condition has no price")?
                .text().collect::<Vec<&str>>().join("");
            let mut cleaned_price = price_str.replace("€", "");
            cleaned_price = cleaned_price.trim().to_string();
            let split: Vec<&str> = cleaned_price.split(",").collect();
            let euros: isize = split.get(0).context("failed to parse price (stage 1e)")?.parse()
                .context("failed to parse price (stage 2e)")?;
            let cents: isize = split.get(1).context("failed to parse price (stage 1c)")?.parse()
                .context("failed to parse price (stage 2c)")?;
            let price = Currency::from_euros_and_cents(euros, cents);

            out.push(Condition {
                name,
                available: true,
                price
            });
        }
        
        Ok(out)
    }
}

#[async_trait(?Send)]
impl Source for Sanomapro {
    fn get_store_name(&self) -> &'static str {
        "Sanomapro"
    }

    fn get_store_url(&self) -> &'static str {
        "https://tuotteet.sanomapro.fi/"
    }

    async fn get_page_url(&self, book_name: &String) -> String {
        format!("https://www.sanomapro.fi/haku/?q={}", book_name)
    }

    async fn parse_document(&self, document: scraper::Html, book_name: &String, cache: &Option<&mut Cache>) -> Response {
        let mut out = vec![];

        let key = self.extract_key(document)?;
        //println!("Sanomapro key: {}", key);

        let new_url = format!("{}{}?term={}&fuzzy=auto&page=1&limit=3&sort=relevance&order=desc", SEARCH_API_URL_BASE, key, book_name);
        //println!("Url: {}", new_url);
        //println!("requesting page...");
        let html = crate::get_page_html(&new_url, cache).await?;
        let data = crate::parse_json(&html)?;

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
                                            let Some(url) = map.get("url") &&
                                            let Some(title) = map.get("title") &&
                                            let Some(images) = map.get("images")
                                        {
                                            if 
                                                let serde_json::Value::String(id) = id &&
                                                let serde_json::Value::String(url) = url &&
                                                let serde_json::Value::String(title) = title &&
                                                let serde_json::Value::Object(images) = images
                                            {
                                                // Get prices
                                                let conditions = self.get_conditions(url, cache).await;

                                                match conditions {
                                                    Ok(conditions) => {
                                                        let mut image: Option<String> = None;
                                                        if images.len() > 0 {
                                                            if let Some(imgval) = images.get("main") {
                                                                if let serde_json::Value::String(url) = imgval {
                                                                    image = Some(url.to_string());
                                                                }
                                                            }
                                                        }
                                                        out.push(Kirja {
                                                            name: title.to_string(),
                                                            id: id.to_string(),
                                                            links: Links {
                                                                buy: url.to_string(),
                                                                image
                                                            },
                                                            source: self.get_store_url().to_string(),
                                                            conditions
                                                        })
                                                    },
                                                    Err(_err) => {},
                                                }
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                        _ => {},
                    }
                }
            },
            _ => {},
        }

        Ok(out)
    }
}