use crate::{
    datum::{auxiliary::HasKey, Datum},
    record::{BillingMethod, Currency},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    name: String,
    address: String,
    currency: Currency,
    billing: BillingMethod,
}

impl Datum for Client {
    const STORE: &'static str = "clients";
}

impl HasKey for Client {
    type Key = String;
    const KEY_FIELDS: &'static [&'static str] = &["name"];
}
