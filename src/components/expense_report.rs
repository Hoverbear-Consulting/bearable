use crate::components::auxililary::{Interactive, List};
use crate::scope::Scope;
use crate::{
    components::{auxililary::Create, Component},
    structures::ExpenseReport,
};
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, ArgMatches};
use dialoguer::Input;
use tracing::{field, trace};

impl Component for ExpenseReport {
    fn app() -> App<'static, 'static> {
        App::new("expense-report")
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

impl Interactive for ExpenseReport {
    fn interactive() -> Result<Self> {
        let theme = dialoguer::theme::ColorfulTheme::default();
        Ok(Self {
            client: (
                (),
                Input::with_theme(&theme)
                    .with_prompt("Client name")
                    .interact()?,
            )
                .into(),
            number: Input::with_theme(&theme).with_prompt("number").interact()?,
            date_issued: Input::with_theme(&theme)
                .with_prompt("billing")
                .interact()?,
            lines: Vec::default(),
        })
    }
}
