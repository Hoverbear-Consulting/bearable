use crate::components::auxililary::Interactive;
use crate::scope::Scope;
use crate::{
    components::{auxililary::Create, Component},
    structures::ExpenseReport,
};
use anyhow::Result;
use clap::{App, AppSettings, ArgMatches};
use tracing::{field, trace};

impl Component for ExpenseReport {
    fn app() -> App<'static, 'static> {
        App::new("expense-report")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![<Create<Self> as Component>::app()])
    }

    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        unimplemented!()
    }
}

impl Interactive for ExpenseReport {
    fn interactive() -> Result<Self> {
        unimplemented!()
    }
}
