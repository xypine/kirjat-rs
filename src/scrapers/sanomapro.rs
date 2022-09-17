use anyhow::{Context, Result};

use super::Scraper;

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
            .collect::<Vec<&str>>().get(1).context("couldn't extract search key")?
            .split("&").collect::<Vec<&str>>();
        let key = *key_step.get(0).context("couldn't extract search key")?;
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

    fn parse_document(&self, document: scraper::Html) -> anyhow::Result<Vec<crate::structs::kirja::Kirja>> {
        let key = self.extract_key(document)?;
        println!("Sanomapro key: {}", key);

        todo!()
    }
}