use crate::datatypes::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LineItem {
    description: String,
    quantity: usize,
    unit_price: Money,
}
