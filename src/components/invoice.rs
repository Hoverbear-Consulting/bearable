use crate::{components::{Client, Component}, datatypes::LineItem};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{info, trace};

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice;

impl Component for Invoice {
    const STORE: Option<&'static str> = Some("invoices");

    fn app() -> App<'static, 'static> {
        App::new("invoice")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![
                App::new("create"),
                App::new("delete"),
                App::new("edit"),
            ])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        trace!("Got args: {:#?}", args);
        match args.subcommand() {
            ("create", Some(args)) => unimplemented!(),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}
