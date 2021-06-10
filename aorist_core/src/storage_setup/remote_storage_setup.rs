use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use crate::encoding::{Encoding};
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
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
            uuid: None,
        }
    }
}
