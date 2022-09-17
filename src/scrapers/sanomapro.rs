use anyhow::{Context, Result, Ok};

use crate::structs::kirja::{Kirja, Links};

use super::Scraper;

const SEARCH_API_URL_BASE: &str = "https://api.addsearch.com/v1/search/";

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
}

impl Scraper for Sanomapro {
    fn get_store_name(&self) -> &'static str {
        "Sanomapro"
    }

    fn get_store_url(&self) -> &'static str {
        "https://tuotteet.sanomapro.fi/"
    }

    fn get_page_url(&self, book_name: &String) -> String {
        format!("https://www.sanomapro.fi/haku/?q={}", book_name)
    }

    fn parse_document(&self, document: scraper::Html, book_name: &String) -> anyhow::Result<Vec<crate::structs::kirja::Kirja>> {
        let mut out = vec![];

        let key = self.extract_key(document)?;
        println!("Sanomapro key: {}", key);

        let new_url = format!("{}{}?term={}&fuzzy=auto&page=1&limit=3&sort=relevance&order=desc", SEARCH_API_URL_BASE, key, book_name);
        println!("Url: {}", new_url);
        println!("requesting page...");
        let html = crate::get_page_html(&new_url)?;
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
                                                    ..Default::default()
                                                })
                                            }
                                        }
                                    },
                                    _ => todo!()
                                }
                            }
                        }
                        _ => todo!(),
                    }
                }
            },
            _ => todo!(),
        }

        Ok(out)
    }
}