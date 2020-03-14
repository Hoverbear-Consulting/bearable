use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod auxiliary;

mod client;
pub use client::Client;

mod invoice;
pub use invoice::Invoice;

mod root;
pub use root::Root;

mod expense_report;
pub use expense_report::ExpenseReport;
use serde::de::DeserializeOwned;
use std::any::Any;

pub trait Datum: Serialize + DeserializeOwned + Debug + Clone {
    const STORE: &'static str;
}
