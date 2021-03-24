#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use markdown_gen::markdown::*;
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
    pub fn markdown(&self, md: &mut Markdown<Vec<u8>>) {
        md.write(
            "RemoteStorageSetup:"
                .bold()
                .paragraph()
                .append(" the dataset is known to be stored in a remote location."),
        )
        .unwrap();
        self.remote.markdown(md);
    }
}

impl InnerRemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: InnerStorage,
        tmp_dir: String,
    ) -> InnerReplicationStorageSetup {
        InnerReplicationStorageSetup {
            source: self.remote.clone(),
            targets: vec![t],
            tag: self.tag.clone(),
            tmp_dir,
        }
    }
}
