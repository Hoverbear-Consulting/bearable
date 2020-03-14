use crate::components::Component;
use crate::datum::Datum;
use crate::Dataset;
use anyhow::Result;
use clap::{App, ArgMatches};
use tracing::{field, info, trace};

pub struct List<D: Datum>(D);

impl<D: Datum> Component for List<D> {
    fn app() -> App<'static, 'static> {
        App::new("list")
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));
        let dataset = Vec::<D>::unstow()?;
        trace!(dataset = field::debug(dataset.clone()));
        Vec::<D>::stow(dataset)?;
        Ok(())
    }
}
