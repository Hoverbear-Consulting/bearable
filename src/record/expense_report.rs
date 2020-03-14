use super::{Client, Datum};
use crate::datum::auxiliary::HasKey;
use crate::record::LineItem;
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
    type Key = (String, usize);
    const KEY_FIELDS: &'static [&'static str] = &["customer", "number"];
}
