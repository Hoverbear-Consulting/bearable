use crate::components::auxililary::Interactive;
use crate::components::Component;
use crate::datum::Datum;
use crate::Dataset;
use anyhow::Result;
use clap::{App, ArgMatches};
use dialoguer::Input;
use tracing::{field, trace};

pub struct Create<D: Datum + Interactive>(D);

impl<D: Datum + Interactive> Component for Create<D> {
    fn app() -> App<'static, 'static> {
        App::new("create")
    }

    fn handle(args: &ArgMatches) -> Result<()> {
        let mut new = D::interactive()?;

        trace!(args = field::debug(args));
        let mut dataset = Vec::<D>::unstow()?;

        trace!(pre = field::debug(dataset.clone()));
        dataset.push(new);
        trace!(post = field::debug(dataset.clone()));

        Vec::<D>::stow(dataset)?;
        Ok(())
    }
}
