#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use crate::constraint::Constraint;
use crate::encoding::{Encoding, InnerEncoding};
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct RemoteStorageSetup {
    #[constrainable]
    pub remote: Storage,
}
impl RemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: Storage,
        tmp_dir: String,
        tmp_encoding: Encoding,
    ) -> ReplicationStorageSetup {
        ReplicationStorageSetup {
            source: self.remote.clone(),
            targets: vec![t],
            tag: self.tag.clone(),
            tmp_dir,
            tmp_encoding,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}

impl InnerRemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: InnerStorage,
        tmp_dir: String,
        tmp_encoding: InnerEncoding,
    ) -> InnerReplicationStorageSetup {
        InnerReplicationStorageSetup {
            source: self.remote.clone(),
            targets: vec![t],
            tag: self.tag.clone(),
            tmp_dir,
            tmp_encoding,
        }
    }
}
