use super::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LineItem {
    number: usize,
    description: String,
    quantity: usize,
    unit_price: Money,
}
