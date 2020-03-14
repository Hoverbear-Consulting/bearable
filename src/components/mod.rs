use anyhow::{anyhow, Result};
use clap::{App, ArgMatches};

mod auxililary;
mod client;
mod expense_report;
mod invoice;
mod root;

pub trait Component {
    fn app() -> App<'static, 'static>;
    fn handle(args: &ArgMatches) -> Result<()>;
    fn execute() -> Result<()> {
        let matches = Self::app().get_matches();
        Self::handle(&matches)
    }
}
