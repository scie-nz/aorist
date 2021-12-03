use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{asset_enum, AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use uuid::Uuid;

asset_enum! {
    name: LanguageAsset
    variants:
    - FasttextEmbedding
    - TextCorpus
    - NamedEntities
}
impl LanguageAsset {
    pub fn get_source_assets(&self) -> Vec<Asset> {
        let source_schema = match &*self.get_schema().0.read() {
            DataSchema::LanguageAssetSchema(x) => x.0.read().get_source_schema(),
            _ => panic!("schema must be LanguageAssetSchema"),
        };
        let sources = source_schema.0.read().get_sources();
        sources
    }
}
