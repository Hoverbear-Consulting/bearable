use crate::{
    datum::{auxiliary::HasKey, Datum},
    record::{BillingMethod, Currency},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    pub name: String,
    pub address: String,
    pub currency: Currency,
    pub billing: BillingMethod,
}

impl Datum for Client {
    const STORE: &'static str = "clients";
}

impl HasKey for Client {
    type Key = String;
    const KEY_FIELDS: &'static [&'static str] = &["name"];
}
