use super::Currency;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tracing::trace;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money {
    /// In the smallest denomination! Eg Cents
    amount: u128,
    currency: Currency,
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(&self).unwrap())
    }
}

impl FromStr for Money {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        trace!(source = s);
        let value = serde_json::from_str(s)?;
        Ok(value)
    }
}
