use scraper::{Selector, ElementRef};
use anyhow::{Result, Context};

use super::Source;
use crate::{structs::{kirja::{Condition, Kirja, Links}, currency::Currency}, Cache};


#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Otava {

}

impl Otava {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Source for Otava {

    fn get_store_name(&self) -> &'static str {
        "Otava"
    }

    fn get_store_url(&self) -> &'static str {
        "https://otava.kauppakv.fi"
    }

    fn get_page_url(&self, book_name: &String) -> String {
        format!("https://otava.kauppakv.fi/sivu/tuotehaku/?action=search&search={}&sortmode=score", book_name)
    }

    fn parse_document(&self, document: scraper::Html, _book_name: &String, _cache: &Option<&mut Cache>) -> Result<Vec<crate::structs::kirja::Kirja>> {
        let mut out = vec![];

        let table_selector = Selector::parse("div.product-list")
            .expect("Failed to construct selector");

        let mut tables = document.select(&table_selector);
        let table = tables.next().context("No elements found")?;

        let product_selector = Selector::parse("div.product-list-div")
            .expect("Failed to construct selector");
        
        let products = table.select(&product_selector);

        let name_selector = Selector::parse(".product-list-content b a")
            .expect("Failed to construct selector");
        let id_selector = Selector::parse("ul.product-list")
                .expect("Failed to construct selector");
        let image_selector = Selector::parse(".product-list-image img")
            .expect("Failed to construct selector");
        let price_selector = Selector::parse(".product-price")
            .expect("Failed to construct selector");
        
        for product in products {
            let ids: Vec<ElementRef> = product.select(&id_selector).collect();
            let names: Vec<ElementRef> = product.select(&name_selector).collect();
            let images: Vec<ElementRef> = product.select(&image_selector).collect();
            let price_container: Vec<ElementRef> = product.select(&price_selector).collect();
            
            if ids.len() > 0 && names.len() > 0 {
                let id: String = ids[0].text().collect::<Vec<&str>>().join("")
                    .chars().filter(|c| c.is_digit(10)).collect();
                let name = names[0].text().collect::<Vec<&str>>().join("");

                //println!("{}: {}", id, name);

                let mut conditions: Vec<Condition> = vec![];
                for condition_container in price_container {
                    let prices: Vec<&str> = condition_container.text().collect();
                    // println!("{:?}", prices);
                    if prices.len() > 0 {
                        let current_price = *prices.last().unwrap();
                        let mut cleaned_price = current_price.replace("€", "");
                        cleaned_price = cleaned_price.trim().to_string();
                        let split: Vec<&str> = cleaned_price.split(",").collect();
                        if split.len() > 1 {
                            let euros: isize = split[0].parse()
                                .context("failed to parse price (e)")?;
                            let cents: isize = split[1].parse()
                                .context("failed to parse price (c)")?;
                            let price = Currency::from_euros_and_cents(euros, cents);
                            conditions.push(Condition {
                                name: "Vakio".to_string(),
                                price,
                                available: true
                            });
                            //print!("{}\t", price);
                        }
                    }
                }
                //println!("");

                let source = self.get_store_url().to_string();
                let buy_link_href = names[0].value().attr("href").context("could not find store href")?;
                let buy_link = format!("{}{}", self.get_store_url(), buy_link_href);
                let image_link: Option<String>;
                if let Some(image_element) = images.first() {
                    let href = image_element.value().attr("src").context("could not find image href")?;
                    image_link = Some(format!("{}/{}", self.get_store_url(), href));
                }
                else {
                    image_link = None;
                }
                let links = Links {
                    buy: buy_link,
                    image: image_link
                };
                out.push(Kirja {
                    id,
                    name,
                    conditions,
                    source,
                    links
                })
            }
        }

        Ok(out)
    }
}