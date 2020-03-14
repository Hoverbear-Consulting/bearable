use crate::datatypes::{BillingMethod, Currency};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    name: String,
    address: String,
    currency: Currency,
    billing: BillingMethod,
}
