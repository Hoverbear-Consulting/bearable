use anyhow::anyhow;
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use tracing::trace;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum BillingMethod {
    /// ACH
    AutomatedClearingHouse,
    /// Interac E-Transfer
    Interac,
    /// From Transferwise
    Transferwise,
    /// The cold, solid stuff
    Cash,
}

impl Display for BillingMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let s = Vec::new();
        let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(s);
        writer.serialize(self).unwrap();
        f.write_str(
            &String::from_utf8(writer.into_inner().unwrap())
                .unwrap()
                .trim(),
        )
    }
}

impl FromStr for BillingMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        trace!(source = s,);
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(s.as_bytes());
        reader
            .deserialize()
            .next()
            .ok_or(anyhow!("Didn't get a value I could parse."))
            .and_then(|v| v.map_err(|e| e.into()))
    }
}
