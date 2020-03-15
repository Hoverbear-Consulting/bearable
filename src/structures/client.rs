use crate::{
    field::{BillingMethod, Currency},
    structures::Structure,
};
use retriever::traits::record::Record;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Client {
    pub name: String,
    pub address: String,
    pub currency: Currency,
    pub billing: BillingMethod,
}

impl Structure for Client {
    const STORE: &'static str = "clients";
    type ChunkKeys = ();
    type ItemKeys = String;
}

impl Record<<Self as Structure>::ChunkKeys, <Self as Structure>::ItemKeys> for Client {
    fn chunk_key(&self) -> Cow<()> {
        Cow::Owned(())
    }

    fn item_key(&self) -> Cow<String> {
        Cow::Borrowed(&self.name)
    }
}
