use super::{Client, Structure};
use crate::field::{ForeignKey, LineItem};
use chrono::{DateTime, Utc};
use retriever::traits::record::Record;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Invoice {
    pub client: ForeignKey<Client>,
    pub number: usize,
    pub date_issued: DateTime<Utc>,
    pub lines: Vec<LineItem>,
}

impl Structure for Invoice {
    const STORE: &'static str = "invoices";
    type ChunkKeys = ForeignKey<Client>;
    type ItemKeys = usize;
}

impl Record<<Self as Structure>::ChunkKeys, <Self as Structure>::ItemKeys> for Invoice {
    fn chunk_key(&self) -> Cow<<Self as Structure>::ChunkKeys> {
        Cow::Owned(self.client.clone().into())
    }

    fn item_key(&self) -> Cow<<Self as Structure>::ItemKeys> {
        Cow::Borrowed(&self.number)
    }
}
