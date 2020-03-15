use crate::scope::Scope;
use crate::{
    components::Component,
    structures::{Client, ExpenseReport, Invoice, Root},
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
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

    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        info!("Welcome to Bearable!");
        match args.subcommand() {
            ("client", Some(args)) => Client::handle(scope, args),
            ("invoice", Some(args)) => Invoice::handle(scope, args),
            ("expense-report", Some(args)) => ExpenseReport::handle(scope, args),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}
