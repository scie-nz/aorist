use crate::encoding::Encoding;
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct RemoteStorageSetup {
    #[constrainable]
    pub remote: AoristRef<Storage>,
}
impl RemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> ReplicationStorageSetup {
        ReplicationStorageSetup {
            source: self.remote.clone(),
            targets: vec![t],
            tag: self.tag.clone(),
            tmp_dir,
            tmp_encoding,
            uuid: None,
        }
    }
}
