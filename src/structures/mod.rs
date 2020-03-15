pub use expense_report::ExpenseReport;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub mod auxiliary;

mod client;
pub use client::Client;

mod invoice;
pub use invoice::Invoice;

mod root;
use retriever::traits::record::Record;
pub use root::Root;
use std::hash::Hash;

mod expense_report;

pub trait Structure:
    Serialize + DeserializeOwned + Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
    const STORE: &'static str;
    type ChunkKeys: Serialize
        + DeserializeOwned
        + Debug
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash;
    type ItemKeys: Serialize
        + DeserializeOwned
        + Debug
        + Clone
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash;
}
