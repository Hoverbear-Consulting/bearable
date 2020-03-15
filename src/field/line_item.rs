use super::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineItem {
    number: usize,
    description: String,
    quantity: usize,
    unit_price: Money,
}
