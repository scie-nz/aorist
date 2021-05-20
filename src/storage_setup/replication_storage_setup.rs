#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::Constraint;
use crate::encoding::*;
use crate::storage::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct ReplicationStorageSetup {
    #[constrainable]
    pub source: Storage,
    #[constrainable]
    pub targets: Vec<Storage>,
    pub tmp_dir: String,
    #[constrainable]
    pub tmp_encoding: Encoding,
}

impl ReplicationStorageSetup {
    pub fn get_download_extension(&self) -> String {
        match self.source.get_encoding() {
            Some(source_encoding) => {
                if source_encoding.is_same_variant_in_enum_as(&self.tmp_encoding) {
                    return source_encoding.get_default_file_extension();
                } else {
                    return "downloaded".to_string();
                }
            }
            None => panic!("get_download_extension called against source storage without encoding"),
        }
    }
}
