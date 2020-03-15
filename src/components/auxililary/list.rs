use crate::components::Component;
use crate::dataset::Dataset;
use crate::scope::Scope;
use crate::structures::Structure;
use anyhow::{anyhow, Result};
use clap::{App, ArgMatches};
use retriever::traits::record::Record;
use retriever::types::storage::Storage;
use tracing::{field, trace};

pub struct List<D: Structure>(D);

impl<D: 'static + Structure + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>>
    Component for List<D>
{
    fn app() -> App<'static, 'static> {
        App::new("list")
    }

    fn handle(scope: &mut Scope, args: &ArgMatches) -> Result<()> {
        trace!(args = field::debug(args));

        let dataset: &Storage<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys, D> = scope
            .store
            .datasets
            .get()
            .ok_or(anyhow!("Failed to get dataset."))?;
        let data = dataset.iter().collect::<Vec<&D>>();
        println!("{:?}", data);

        Ok(())
    }
}
