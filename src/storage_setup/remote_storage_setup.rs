#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
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

impl InnerRemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: InnerStorage,
        tmp_dir: String,
    ) -> InnerReplicationStorageSetup {
        InnerReplicationStorageSetup {
            remote: self.remote.clone(),
            local: vec![t],
            tag: self.tag.clone(),
            tmp_dir,
        }
    }
}
