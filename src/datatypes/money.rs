use crate::datatypes::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Money {
    /// In the smallest denomination! Eg Cents
    amount: u128,
    currency: Currency,
}
