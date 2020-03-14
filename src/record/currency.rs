use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use tracing::{field, trace};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Currency {
    Canadian,
    American,
    Chinese,
    European,
    British,
}

impl Display for Currency {
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

impl FromStr for Currency {
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
