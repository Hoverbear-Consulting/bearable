use crate::structures::Structure;
use anyhow::Result;
use retriever::traits::record::Record;
use retriever::types::storage::Storage;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Seek};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use tracing::{field, info, trace};

const STASH: &str = "data";

pub trait Dataset<D>:
    Serialize + DeserializeOwned + Debug + Default + FromIterator<D> + IntoIterator<Item = D> + Clone
where
    D: Structure + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>,
{
    const STORE: &'static str;

    fn store_path() -> PathBuf {
        Path::new(STASH).join(Self::STORE).with_extension("json")
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
        trace!(opening = field::debug(Self::store_path()));
        let handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(store_path)?;
        info!("Writing...");
        serde_json::to_writer_pretty(handle, &self)?;
        Ok(())
    }

    fn unstow() -> Result<Self> {
        Self::ensure_store_dir()?;
        trace!(opening = field::debug(Self::store_path()));
        let handle = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(Self::store_path())?;
        let dataset = if handle.metadata()?.len() != 0 {
            let dataset: Self = serde_json::from_reader(handle)?;
            dataset
        } else {
            Self::default()
        };
        Ok(dataset)
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
