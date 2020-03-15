use crate::{
    field::{BillingMethod, Currency},
    record::{auxiliary::HasKey, Datum},
};
use retriever::traits::record::Record;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

impl Record<(), String> for Client {
    fn chunk_key(&self) -> Cow<'a, ()> {
        unimplemented!()
    }

    fn item_key(&self) -> Cow<'a, String> {
        unimplemented!()
    }
}
