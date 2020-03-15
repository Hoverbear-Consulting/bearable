use crate::store::Store;
use anyhow::Result;
use clap::ArgMatches;

pub struct Scope {
    pub(crate) store: Store,
}

impl Scope {
    pub fn init(_args: &ArgMatches) -> Result<Self> {
        Ok(Self {
            store: Store::init()?,
        })
    }
}
