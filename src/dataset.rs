use crate::structures::Structure;
use anyhow::Result;
use retriever::traits::record::Record;
use retriever::types::storage::Storage;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::fs::{create_dir_all, OpenOptions};
use std::hash::Hash;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use tracing::{field, info, trace};

const STASH: &str = "data";

pub trait Dataset<D>:
    Serialize + DeserializeOwned + Debug + FromIterator<D> + IntoIterator<Item = D> + Clone
where
    D: Structure + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>,
{
    const STORE: &'static str;

    fn store_path() -> PathBuf {
        Path::new(STASH).join(Self::STORE).with_extension("csv")
    }

    fn ensure_store_dir() -> Result<()> {
        let store_dir = Self::store_path().parent().unwrap().to_path_buf();
        trace!(create = field::debug(store_dir.clone()));
        create_dir_all(store_dir)?;
        Ok(())
    }

    fn stow(self) -> Result<()> {
        Self::ensure_store_dir()?;
        let store_path = Self::store_path();
        let handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(store_path)?;
        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(handle);
        info!("Writing...");
        for item in self {
            writer.serialize(item)?;
        }
        Ok(())
    }

    fn unstow() -> Result<Self> {
        Self::ensure_store_dir()?;
        let handle = OpenOptions::new().read(true).open(Self::store_path())?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(handle);
        info!("Reading...");
        reader
            .deserialize()
            .map(|v| match v {
                Ok(v) => {
                    // This is not actually a no-op!
                    let v: D = v;
                    trace!(item = field::debug(v.clone()));
                    Ok(v)
                }
                Err(e) => Err(e.into()),
            })
            .collect::<Result<Self>>()
    }

    fn storage() -> Result<Storage<D::ChunkKeys, D::ItemKeys, D>> {
        let dataset = Self::unstow()?;
        trace!(dataset_size = field::debug(dataset.clone()));
        let mut storage = Storage::new();
        for datum in dataset {
            trace!(item_added = field::debug(datum.clone()));
            storage.add(datum);
        }
        Ok(storage)
    }

    fn from_storage(storage: Storage<D::ChunkKeys, D::ItemKeys, D>) -> Self {
        storage.iter().cloned().collect()
    }
}

impl<D: Structure + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>> Dataset<D>
    for Vec<D>
{
    const STORE: &'static str = D::STORE;
}
