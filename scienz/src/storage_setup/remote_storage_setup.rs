use crate::encoding::Encoding;
use crate::storage::*;
use crate::storage_setup::replication_storage_setup::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct RemoteStorageSetup {
    #[constrainable]
    pub remote: AoristRef<Storage>,
    pub tmp_dir: AOption<AString>,
}
impl RemoteStorageSetup {
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: AString,
        tmp_encoding: AoristRef<Encoding>,
    ) -> ReplicationStorageSetup {
        ReplicationStorageSetup {
            source: self.remote.clone(),
            targets: vec![t].into_iter().collect(),
            tag: self.tag.clone(),
            tmp_dir,
            tmp_encoding,
            uuid: AOption(ROption::RNone),
        }
    }
}
