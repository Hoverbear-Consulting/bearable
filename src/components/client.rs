use crate::{
    components::{
        auxililary::{Create, List},
        Component,
    },
    datum::Client,
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{field, trace};

impl Component for Client {
    fn app() -> App<'static, 'static> {
        App::new("client")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![Create::<Self>::app(), List::<Self>::app()])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        match args.subcommand() {
            ("create", Some(args)) => <Create<Self> as Component>::handle(args),
            ("list", Some(args)) => <List<Self> as Component>::handle(args),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}

impl Create<Client> {}
