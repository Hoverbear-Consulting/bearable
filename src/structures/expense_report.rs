use super::{Client, Structure};
use crate::field::LineItem;
use retriever::traits::record::Record;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpenseReport {
    client: (
        <Client as Structure>::ChunkKeys,
        <Client as Structure>::ItemKeys,
    ),
    number: usize,
    lines: Vec<LineItem>,
}

impl Structure for ExpenseReport {
    const STORE: &'static str = "expense-reports";
    type ChunkKeys = (
        <Client as Structure>::ChunkKeys,
        <Client as Structure>::ItemKeys,
    );
    type ItemKeys = usize;
}

impl Record<<Self as Structure>::ChunkKeys, <Self as Structure>::ItemKeys> for ExpenseReport {
    fn chunk_key(&self) -> Cow<<Self as Structure>::ChunkKeys> {
        Cow::Borrowed(&self.client)
    }

    fn item_key(&self) -> Cow<<Self as Structure>::ItemKeys> {
        Cow::Borrowed(&self.number)
    }
}
