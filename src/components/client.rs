use crate::scope::Scope;
use crate::{
    components::{
        auxililary::{Create, Interactive, List},
        Component,
    },
    structures::Client,
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, ArgMatches};
use dialoguer::Input;
use tracing::{field, trace};

impl Component for Client {
    fn app() -> App<'static, 'static> {
        App::new("client")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![Create::<Self>::app(), List::<Self>::app()])
    }

    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        match args.subcommand() {
            ("create", Some(args)) => <Create<Self> as Component>::handle(scope, args),
            ("list", Some(args)) => <List<Self> as Component>::handle(scope, args),
            _ => Err(anyhow!("Invalid subcommand.")),
        }
    }
}

impl Interactive for Client {
    fn interactive() -> Result<Self> {
        let theme = dialoguer::theme::ColorfulTheme::default();
        Ok(Self {
            name: Input::with_theme(&theme).with_prompt("name").interact()?,
            address: Input::with_theme(&theme)
                .with_prompt("address")
                .interact()?,
            currency: Input::with_theme(&theme)
                .with_prompt("currency")
                .interact()?,
            billing: Input::with_theme(&theme)
                .with_prompt("billing")
                .interact()?,
        })
    }
}
