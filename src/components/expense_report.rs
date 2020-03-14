use crate::components::auxililary::Interactive;
use crate::{
    components::{auxililary::Create, Component},
    datum::ExpenseReport,
};
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{field, trace};

impl Component for ExpenseReport {
    fn app() -> App<'static, 'static> {
        App::new("expense-report")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![<Create<Self> as Component>::app()])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        unimplemented!()
    }
}

impl Interactive for ExpenseReport {
    fn interactive() -> Result<Self> {
        unimplemented!()
    }
}
