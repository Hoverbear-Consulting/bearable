use super::{Client, Datum};
use crate::field::LineItem;
use crate::record::auxiliary::HasKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpenseReport {
    customer: Client,
    number: usize,
    lines: Vec<LineItem>,
}

impl Datum for ExpenseReport {
    const STORE: &'static str = "expense-reports";
}

impl HasKey for ExpenseReport {
    type ChunkKeys = Client;
    const CHUNK_KEY_FIELDS: &'static [&'static str] = &["customer"];
    type ItemKeys = usize;
    const ITEM_KEY_FIELDS: &'static [&'static str] = &["number"];
}
