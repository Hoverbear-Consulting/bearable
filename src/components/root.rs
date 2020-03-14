use serde::{Deserialize, Serialize};
use clap::{App, AppSettings, Arg, ArgMatches};
use anyhow::{anyhow, Result};
use crate::components::{Component, Client, Invoice, ExpenseReport};
use crate::observability;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct Root;

impl Component for Root {
    const STORE: Option<&'static str> = None;

    fn app() -> App<'static, 'static> {
        App::new("Billing")
            .global_settings(&[AppSettings::ColoredHelp])
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .arg(Arg::with_name("log")
                .short("l")
                .long("log")
                .value_name("LEVEL")
                .possible_values(&[
                    "ERROR",
                    "WARN",
                    "INFO",
                    "DEBUG",
                    "TRACE",
                ])
                .takes_value(true))
            .subcommands(vec![Client::app(), Invoice::app(), ExpenseReport::app()])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        crate::observability::init(args.value_of("log"));
        info!("Welcome to billing!");
        match args.subcommand() {
            ("client", Some(args)) => Client::handle(args),
            ("invoice", Some(args)) => Invoice::handle(args),
            ("expense-report", Some(args)) => ExpenseReport::handle(args),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}