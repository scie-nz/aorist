use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TabularCollectionSchema {
    // same datum_template as a TabularSchema
    pub datum_template: AoristRef<DatumTemplate>,
    pub source_assets: AVec<AoristRef<Asset>>,
    pub attributes: AVec<AString>,
}
impl TabularCollectionSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        let mut attributes_map = LinkedHashMap::new();
        for asset in self.source_assets.iter() {
            let mut asset_attr: HashMap<AString, Attribute> = asset
                .0
                .read()
                .get_schema()
                .0
                .read()
                .get_attributes()
                .into_iter()
                .map(|x| {
                    let attribute: Attribute = x.0.read().clone();
                    (attribute.get_name().clone(), attribute)
                })
                .collect();
            for attribute_name in self.attributes.clone() {
                assert!(asset_attr.contains_key(&attribute_name));
                if attributes_map.len() < self.attributes.len() {
                    attributes_map.insert(
                        attribute_name.clone(),
                        AoristRef(RArc::new(RRwLock::new(
                            asset_attr.remove(&attribute_name).unwrap(),
                        ))),
                    );
                } else {
                    assert_eq!(
                        *attributes_map.get(&attribute_name).unwrap().0.read(),
                        asset_attr.remove(&attribute_name).unwrap(),
                    );
                }
            }
        }
        attributes_map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTabularCollectionSchema {
    #[getter]
    pub fn get_source_assets(&self) -> Vec<PyAsset> {
        self.inner
            .0
            .read()
            .source_assets
            .iter()
            .map(|x| PyAsset { inner: x.clone() })
            .collect()
    }
}
