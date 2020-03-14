use crate::{
    datatypes::{BillingMethod, Currency, LineItem},
    components::Component,
};
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Client;

impl Component for Client {
    const STORE: Option<&'static str> = None;

    fn app() -> App<'static, 'static> {
        App::new("client")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![
                App::new("create").arg(Arg::with_name("name")),
                App::new("delete"),
                App::new("list"),
                App::new("edit"),
            ])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        unimplemented!()
    }
}
