use crate::components::auxililary::Interactive;
use crate::{
    components::{auxililary::Create, Component},
    datum::Invoice,
};
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{field, trace};

impl Component for Invoice {
    fn app() -> App<'static, 'static> {
        App::new("invoice")
            .settings(&[AppSettings::SubcommandRequiredElseHelp])
            .subcommands(vec![<Create<Self> as Component>::app()])
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        unimplemented!()
    }
}

impl Interactive for Invoice {
    fn interactive() -> Result<Self> {
        unimplemented!()
    }
}
