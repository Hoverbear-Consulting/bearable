mod client;
pub use client::Client;
mod invoice;
pub use invoice::Invoice;
mod expense_report;
pub use expense_report::ExpenseReport;
mod root;
pub use root::Root;

use clap::{App, ArgMatches};
use anyhow::{anyhow, Result};

pub trait Component {
    const STORE: Option<&'static str>;

    fn app() -> App<'static, 'static>;
    fn handle(args: &ArgMatches) -> Result<()>;
    fn execute() -> Result<()> {
        let matches = Self::app().get_matches();
        Self::handle(&matches)
    }
}