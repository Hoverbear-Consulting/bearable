use super::{Client, Structure};
use crate::field::LineItem;
use chrono::{DateTime, Utc};
use retriever::traits::record::Record;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Invoice {
    client: (
        <Client as Structure>::ChunkKeys,
        <Client as Structure>::ItemKeys,
    ),
    number: usize,
    date_issued: DateTime<Utc>,
    lines: Vec<LineItem>,
}

impl Structure for Invoice {
    const STORE: &'static str = "invoices";
    type ChunkKeys = (
        <Client as Structure>::ChunkKeys,
        <Client as Structure>::ItemKeys,
    );
    type ItemKeys = usize;
}

impl Record<<Self as Structure>::ChunkKeys, <Self as Structure>::ItemKeys> for Invoice {
    fn chunk_key(&self) -> Cow<<Self as Structure>::ChunkKeys> {
        Cow::Borrowed(&self.client)
    }

    fn item_key(&self) -> Cow<<Self as Structure>::ItemKeys> {
        Cow::Borrowed(&self.number)
    }
}
