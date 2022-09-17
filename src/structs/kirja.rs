use serde::{Serialize, Deserialize};

use super::currency::Currency;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct Kirja {
    pub id: String,
    pub source: String,
    pub name: String,
    pub conditions: Vec<Condition>,
    pub links: Links
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub price: Currency,
    pub available: bool
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct Links {
    pub buy: String,
    pub image: Option<String>
}