use scraper::{Selector, ElementRef};
use anyhow::{Result, Context};

use super::Scraper;
use crate::{structs::{kirja::{Condition, Kirja, Links}, currency::Currency}};

pub struct Jamera {

}

impl Jamera {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Scraper for Jamera {

    fn get_store_name(&self) -> &'static str {
        "Jamera"
    }

    fn get_store_url(&self) -> &'static str {
        "https://kauppa.jamera.net"
    }

    fn get_page_url(&self, book_name: &String) -> String {
        format!("https://kauppa.jamera.net/kauppa/haku/?q={}", book_name)
    }

    fn parse_document(&self, document: scraper::Html, _book_name: &String) -> Result<Vec<crate::structs::kirja::Kirja>> {
        let mut out = vec![];

        let table_selector = Selector::parse("table.tuotteet_flex")
            .expect("Failed to construct selector");

        let mut tables = document.select(&table_selector);
        let table = tables.next().context("No elements found")?;

        let product_selector = Selector::parse("tr.tuotteet")
            .expect("Failed to construct selector");
        let _bookseries_selector = Selector::parse("tr.kirjasarja")
            .expect("Failed to construct selector");
        
        let products = table.select(&product_selector);

        let name_selector = Selector::parse("a.otsikko")
            .expect("Failed to construct selector");
        let id_selector = Selector::parse("a[name]")
                .expect("Failed to construct selector");
        let image_selector = Selector::parse("img.tuote_kuva")
            .expect("Failed to construct selector");
        let condition_selector = Selector::parse("td.radio")
            .expect("Failed to construct selector");
        
        for product in products {
            let ids: Vec<ElementRef> = product.select(&id_selector).collect();
            let names: Vec<ElementRef> = product.select(&name_selector).collect();
            let images: Vec<ElementRef> = product.select(&image_selector).collect();
            let condition_containers: Vec<ElementRef> = product.select(&condition_selector).collect();
            
            if ids.len() > 0 && names.len() > 0 {
                let id = ids[0].value().attr("name").context("Couldn't find expected id")?.to_string();
                let name = names[0].text().collect::<Vec<&str>>().join("");

                //println!("{}: {}", id, name);

                let mut conditions: Vec<Condition> = vec![];
                let cname_selector = Selector::parse("p.name")
                    .expect("Failed to construct selector");
                let ccolor_selector = Selector::parse("p.vari")
                    .expect("Failed to construct selector");
                let price_selector = Selector::parse("span")
                    .expect("Failed to construct selector");
                for condition_container in condition_containers {
                    let name_opt = condition_container.select(&cname_selector).next();
                    if let Some(name) = name_opt {
                        let mut color = condition_container.select(&ccolor_selector);
                        
                        let name_to_use: String;
                        if let Some(colorname) = color.next() {
                            name_to_use = colorname.text().collect::<Vec<&str>>().join("");
                        }
                        else {
                            name_to_use = name.text().collect::<Vec<&str>>().join("");
                        }

                        let price_container_selected = condition_container.select(&price_selector);
                        // A valid Condition must have at least one price
                        if let Some(price_container) = price_container_selected.last() {
                            let prices: Vec<&str> = price_container.text().collect();
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
                                        name: name_to_use,
                                        price,
                                        available: true
                                    });
                                    //print!("{}\t", price);
                                }
                            }
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