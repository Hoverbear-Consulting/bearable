use crate::{components::{Client, Component}, datatypes::LineItem};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpenseReport;

impl Component for ExpenseReport {
    const STORE: Option<&'static str> = None;

    fn app() -> App<'static, 'static> {
        App::new("expense-report")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![
                App::new("create"),
                App::new("delete"),
                App::new("edit"),
            ])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        unimplemented!()
    }
}
