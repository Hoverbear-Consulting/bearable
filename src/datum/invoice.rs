use super::{Client, Datum};
use crate::datum::auxiliary::HasKey;
use crate::record::LineItem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Invoice {
    customer: Client,
    number: usize,
    date_issued: DateTime<Utc>,
    lines: Vec<LineItem>,
}

impl Datum for Invoice {
    const STORE: &'static str = "invoices";
}

impl HasKey for Invoice {
    type Key = (String, usize);
    const KEY_FIELDS: &'static [&'static str] = &["customer", "number"];
}
