use crate::components::auxililary::Interactive;
use crate::components::Component;
use crate::scope::Scope;
use crate::structures::Structure;
use anyhow::{anyhow, Result};
use clap::{App, ArgMatches};
use retriever::traits::record::Record;
use retriever::types::storage::Storage;

pub struct Create<D: Structure + Interactive>(D);

impl<
        D: 'static
            + Structure
            + Interactive
            + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>,
    > Component for Create<D>
{
    fn app() -> App<'static, 'static> {
        App::new("create")
    }

    fn handle(scope: &mut Scope, _args: &ArgMatches) -> Result<()> {
        let new = D::interactive()?;

        let dataset: &mut Storage<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys, D> =
            scope
                .store
                .datasets
                .get_mut()
                .ok_or(anyhow!("Failed to get dataset."))?;
        dataset.add(new);

        Ok(())
    }
}
