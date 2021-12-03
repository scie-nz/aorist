#![allow(non_snake_case)]
use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

#[aorist]
pub struct TabularCollectionSchema {
    // same datum_template as a TabularSchema
    pub datum_template: AoristRef<DatumTemplate>,
    pub source_assets: Vec<AoristRef<Asset>>,
    pub attributes: Vec<String>,
}
impl TabularCollectionSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        let mut attributes_map = LinkedHashMap::new();
        for asset in &self.source_assets {
            let mut asset_attr: HashMap<String, Attribute> = asset
                .0
                .read()
                .unwrap()
                .get_schema()
                .0
                .read()
                .unwrap()
                .get_attributes()
                .into_iter()
                .map(|x| {
                    let attribute: Attribute = x.0.read().unwrap().clone();
                    (attribute.get_name().clone(), attribute)
                })
                .collect();
            for attribute_name in self.attributes.clone() {
                assert!(asset_attr.contains_key(&attribute_name));
                if attributes_map.len() < self.attributes.len() {
                    attributes_map.insert(
                        attribute_name.clone(),
                        AoristRef(Arc::new(RwLock::new(
                            asset_attr.remove(&attribute_name).unwrap(),
                        ))),
                    );
                } else {
                    assert_eq!(
                        *attributes_map
                            .get(&attribute_name)
                            .unwrap()
                            .0
                            .read()
                            .unwrap(),
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
            .unwrap()
            .source_assets
            .iter()
            .map(|x| PyAsset { inner: x.clone() })
            .collect()
    }
}
