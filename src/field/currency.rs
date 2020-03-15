use crate::field::HasVariants;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tracing::trace;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    Canadian,
    American,
    Chinese,
    European,
    British,
}

pub const CURRENCIES: &[Currency] = &[
    Currency::Canadian,
    Currency::American,
    Currency::Chinese,
    Currency::European,
    Currency::British,
];

impl HasVariants for Currency {
    fn variants() -> Vec<String> {
        CURRENCIES
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&serde_json::to_string(&self).unwrap())
    }
}

impl FromStr for Currency {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        trace!(source = s,);
        let value = serde_json::from_str(s)?;
        Ok(value)
    }
}
