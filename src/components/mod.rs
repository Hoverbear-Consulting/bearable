use crate::scope::Scope;
use anyhow::Result;
use clap::{App, ArgMatches};
use tracing::trace;

mod auxililary;
mod client;
mod expense_report;
mod invoice;
mod root;

pub trait Component {
    fn app() -> App<'static, 'static>;
    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()>;
    fn execute() -> Result<()> {
        let matches = Self::app().get_matches();
        crate::observability::init(matches.value_of("log"))?;
        trace!("Observability started...");
        let mut scope = Scope::init(&matches)?;
        Self::handle(&mut scope, &matches)?;
        scope.store.close();
        Ok(())
    }
}
