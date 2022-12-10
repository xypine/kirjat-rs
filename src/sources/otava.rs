use anyhow::Context;
use async_trait::async_trait;
use scraper::{ElementRef, Selector};

use super::{RequestDetails, Source};
use crate::{
    structs::{
        currency::Currency,
        kirja::{Condition, Kirja, Links},
        response::Response,
    },
    Cache,
};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Otava {}

impl Otava {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl Source for Otava {
    fn get_store_name(&self) -> &'static str {
        "Otava"
    }

    fn get_store_url(&self) -> &'static str {
        "https://otava.kauppakv.fi"
    }

    async fn get_request_details(&self, book_name: &String) -> RequestDetails {
        let url = format!(
            "https://otava.kauppakv.fi/sivu/tuotehaku/?action=search&search={}&sortmode=score",
            book_name
        );
        RequestDetails { url, headers: None }
    }

    async fn parse_document(
        &self,
        plaintext: String,
        _book_name: &String,
        _cache: &Option<&mut Cache>,
    ) -> Response {
        let document = crate::parse_html(&plaintext);
        let mut out = vec![];

        let table_selector =
            Selector::parse("div.product-list").expect("Failed to construct selector");

        let mut tables = document.select(&table_selector);
        let table = tables.next().context("No elements found")?;

        let product_selector =
            Selector::parse("div.product-list-div").expect("Failed to construct selector");

        let products = table.select(&product_selector);

        let name_selector =
            Selector::parse(".product-list-content b a").expect("Failed to construct selector");
        let id_selector = Selector::parse("ul.product-list").expect("Failed to construct selector");
        let image_selector =
            Selector::parse(".product-list-image img").expect("Failed to construct selector");
        let price_selector =
            Selector::parse(".product-price").expect("Failed to construct selector");

        for product in products {
            let ids: Vec<ElementRef> = product.select(&id_selector).collect();
            let names: Vec<ElementRef> = product.select(&name_selector).collect();
            let images: Vec<ElementRef> = product.select(&image_selector).collect();
            let price_container: Vec<ElementRef> = product.select(&price_selector).collect();

            if ids.len() > 0 && names.len() > 0 {
                let id: String = ids[0]
                    .text()
                    .collect::<Vec<&str>>()
                    .join("")
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect();
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
                            let euros: isize =
                                split[0].parse().context("failed to parse price (e)")?;
                            let cents: isize =
                                split[1].parse().context("failed to parse price (c)")?;
                            let price = Currency::from_euros_and_cents(euros, cents);
                            conditions.push(Condition {
                                name: "Vakio".to_string(),
                                price,
                                available: true,
                            });
                            //print!("{}\t", price);
                        }
                    }
                }
                //println!("");

                let source = self.get_store_url().to_string();
                let buy_link_href = names[0]
                    .value()
                    .attr("href")
                    .context("could not find store href")?;
                let buy_link = format!("{}{}", self.get_store_url(), buy_link_href);
                let image_link: Option<String>;
                if let Some(image_element) = images.first() {
                    let href = image_element
                        .value()
                        .attr("data-src")
                        .context("could not find image href")?;
                    image_link = Some(href.to_string());
                } else {
                    image_link = None;
                }
                let links = Links {
                    buy: buy_link,
                    image: image_link,
                };
                out.push(Kirja {
                    id,
                    name,
                    conditions,
                    source,
                    links,
                })
            }
        }

        Response::Ok(out)
    }
}
