use crate::datum::Datum;
use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use tracing::{field, info, trace};

const STASH: &str = "data";

pub trait Dataset<D: Datum>:
    Serialize + DeserializeOwned + Debug + FromIterator<D> + IntoIterator<Item = D>
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
        let mut handle = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(Self::store_path())?;
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
        let mut handle = OpenOptions::new().read(true).open(Self::store_path())?;
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
}

impl<D: Datum> Dataset<D> for Vec<D> {
    const STORE: &'static str = D::STORE;
}
