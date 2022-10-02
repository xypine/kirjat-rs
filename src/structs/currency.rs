use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct Currency {
    /// Currency is tracked in euro cents, use the methods to_euros and to_euros_and_cents if you want it in another form.
    pub euro_cents: isize
}

impl Currency {
    pub fn new(euro_cents: isize) -> Self {
        Self {
            euro_cents
        }
    }

    pub fn from_euros(euros: f64) -> Self {
        let euro_cents = (euros * 100.0) as isize;
        Self {
            euro_cents
        }
    }
    pub fn from_euros_and_cents(euros: isize, cents: isize) -> Self {
        let euro_cents = (euros * 100) + cents;
        Self {
            euro_cents
        }
    }

    pub fn to_euros(&self) -> f64 {
        self.euro_cents as f64 / 100.0
    }
    pub fn to_euros_and_cents(&self) -> (isize, isize) {
        let euros = self.to_euros().floor() as isize;
        let cents = self.euro_cents % 100;
        (euros, cents)
    }
}

use std::fmt::Display;
impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (euros, cents) = self.to_euros_and_cents();
        let formatted = format!("{},{}€", euros, cents);
        f.write_str(&formatted)
    }
}