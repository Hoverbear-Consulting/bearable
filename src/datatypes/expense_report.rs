use serde::{Deserialize, Serialize};
use crate::datatypes::{LineItem, Client};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpenseReport {
    customer: Client,
    lines: Vec<LineItem>,
}
