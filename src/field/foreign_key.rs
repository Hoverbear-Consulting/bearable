use crate::store::Store;
use crate::structures::Structure;
use retriever::traits::record::Record;
use retriever::types::entry::Entry;
use retriever::types::id::Id;
use retriever::types::storage::Storage;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForeignKey<D: Structure> {
    chunk_keys: D::ChunkKeys,
    item_keys: D::ItemKeys,
}

impl<D: Structure> From<(D::ChunkKeys, D::ItemKeys)> for ForeignKey<D> {
    fn from((chunk_keys, item_keys): (D::ChunkKeys, D::ItemKeys)) -> Self {
        Self {
            chunk_keys,
            item_keys,
        }
    }
}

impl<D: Structure> From<ForeignKey<D>> for (D::ChunkKeys, D::ItemKeys) {
    fn from(key: ForeignKey<D>) -> Self {
        (key.chunk_keys, key.item_keys)
    }
}

impl<D: Structure> From<ForeignKey<D>> for Id<D::ChunkKeys, D::ItemKeys> {
    fn from(key: ForeignKey<D>) -> Self {
        Id::new(key.chunk_keys, key.item_keys)
    }
}

impl<D: 'static + Structure + Record<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>>
    ForeignKey<D>
{
    fn follow<'a, R>(
        &self,
        store: &'a mut Store,
    ) -> Entry<
        'a,
        Id<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys>,
        <D as Structure>::ChunkKeys,
        <D as Structure>::ItemKeys,
        D,
    >
    where
        R: retriever::traits::record::Record<
            <D as Structure>::ChunkKeys,
            <D as Structure>::ItemKeys,
        >,
    {
        let dataset = store
            .datasets
            .get_mut::<Storage<<D as Structure>::ChunkKeys, <D as Structure>::ItemKeys, D>>()
            .expect("Expected known dataset");
        let record: Id<D::ChunkKeys, D::ItemKeys> = self.clone().into();
        let datum = dataset.entry(record);
        datum
    }
}

impl<D: Structure> ForeignKey<D> {}
