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
pub use root::Root;

mod expense_report;

pub trait Datum: Serialize + DeserializeOwned + Debug + Clone {
    const STORE: &'static str;
}
