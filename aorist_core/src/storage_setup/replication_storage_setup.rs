use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use crate::encoding::*;
use crate::storage::*;
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
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
