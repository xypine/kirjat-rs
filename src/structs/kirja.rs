use serde::{Deserialize, Serialize};

use super::currency::Currency;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Kirja {
    pub id: String,
    pub source: String,
    pub name: String,
    pub conditions: Vec<Condition>,
    pub links: Links,
}

impl Kirja {
    pub fn get_min_price(&self) -> Option<Currency> {
        let mut conditions = self.conditions.clone();

        conditions.sort();

        let cheapest_condition = conditions.last()?;
        return Some(cheapest_condition.price);
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub price: Currency,
    pub available: bool,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Links {
    pub buy: String,
    pub image: Option<String>,
}
