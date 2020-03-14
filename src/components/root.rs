use crate::{
    components::{auxililary::Create, Component},
    datum::{Client, ExpenseReport, Invoice, Root},
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{field, info, trace};

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new("Bearable")
            .global_settings(&[AppSettings::ColoredHelp])
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .arg(
                Arg::with_name("log")
                    .short("l")
                    .long("log")
                    .value_name("LEVEL")
                    // .possible_values(&["ERROR", "WARN", "INFO", "DEBUG", "TRACE"])
                    .takes_value(true),
            )
            .subcommands(vec![Client::app(), Invoice::app(), ExpenseReport::app()])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        crate::observability::init(args.value_of("log"))?;
        trace!("Observability started...");
        trace!(args = field::debug(args));
        info!("Welcome to Bearable!");
        match args.subcommand() {
            ("client", Some(args)) => Client::handle(args),
            ("invoice", Some(args)) => Invoice::handle(args),
            ("expense-report", Some(args)) => ExpenseReport::handle(args),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}
