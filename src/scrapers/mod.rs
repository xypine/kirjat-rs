use anyhow::Result;

pub trait Scraper {
    fn parse(document: scraper::Html);
}