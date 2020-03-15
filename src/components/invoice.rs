use crate::components::auxililary::Interactive;
use crate::scope::Scope;
use crate::{
    components::{auxililary::Create, Component},
    structures::Invoice,
};
use anyhow::Result;
use clap::{App, AppSettings, ArgMatches};
use tracing::{field, trace};

impl Component for Invoice {
    fn app() -> App<'static, 'static> {
        App::new("invoice")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![<Create<Self> as Component>::app()])
    }

    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        unimplemented!()
    }
}

impl Interactive for Invoice {
    fn interactive() -> Result<Self> {
        unimplemented!();
        // let theme = dialoguer::theme::ColorfulTheme::default();
        // Ok(Self {
        //     client: Input::with_theme(&theme).with_prompt("client").interact()?,
        //     number: Input::with_theme(&theme).with_prompt("number").interact()?,
        //     date_issued: Input::with_theme(&theme)
        //         .with_prompt("Date Issued")
        //         .interact()?,
        //     lines: vec![],
        // })
    }
}
