use crate::dataset::Dataset;
use crate::structures::{Client, ExpenseReport, Invoice, Structure};
use anyhow::Result;
use anymap::AnyMap;
use retriever::types::storage::Storage;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::trace;

pub struct Store {
    pub(crate) datasets: AnyMap,
    // clients: Storage<<Client as Structure>::ChunkKeys, <Client as Structure>::ItemKeys, Client>,
    // invoices: Storage<<Invoice as Structure>::ChunkKeys, <Invoice as Structure>::ItemKeys, Invoice>,
    // expense_reports: Storage<
    //     <ExpenseReport as Structure>::ChunkKeys,
    //     <ExpenseReport as Structure>::ItemKeys,
    //     ExpenseReport,
    // >,
}

impl Store {
    pub fn init() -> Result<Self> {
        trace!("Store initializing");
        Ok(Self {
            datasets: {
                let mut map = AnyMap::new();
                map.insert(Vec::<Client>::storage()?);
                map.insert(Vec::<Invoice>::storage()?);
                map.insert(Vec::<ExpenseReport>::storage()?);
                map
            },
        })
    }

    pub fn close(self) -> Result<()> {
        trace!("Store closing");
        Vec::<Client>::from_storage(
            self.datasets.get::<Storage<
                <Client as Structure>::ChunkKeys,
                <Client as Structure>::ItemKeys,
                Client,
            >>()
                .expect("Invalid store.")
                .clone(),
        ).stow()?;
        Vec::<Invoice>::from_storage(
            self.datasets
                .get::<Storage<
                    <Invoice as Structure>::ChunkKeys,
                    <Invoice as Structure>::ItemKeys,
                    Invoice,
                >>()
                .expect("Invalid store.")
                .clone(),
        )
        .stow()?;
        Vec::<ExpenseReport>::from_storage(
            self.datasets
                .get::<Storage<
                    <ExpenseReport as Structure>::ChunkKeys,
                    <ExpenseReport as Structure>::ItemKeys,
                    ExpenseReport,
                >>()
                .expect("Invalid store.")
                .clone(),
        )
        .stow()?;
        Ok(())
    }
}
