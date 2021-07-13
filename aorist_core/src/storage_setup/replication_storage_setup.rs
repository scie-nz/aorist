use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[aorist]
pub struct ReplicationStorageSetup {
    #[constrainable]
    pub source: AoristRef<Storage>,
    #[constrainable]
    pub targets: Vec<AoristRef<Storage>>,
    pub tmp_dir: String,
    #[constrainable]
    pub tmp_encoding: AoristRef<Encoding>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyReplicationStorageSetup {
    #[getter]
    pub fn download_extension(&self) -> PyResult<String> {
        let rss = &*self.inner.0.read().unwrap();
        Ok(rss.get_download_extension())
    }
}

impl ReplicationStorageSetup {
    pub fn get_download_extension(&self) -> String {
        match self.source.0.read().unwrap().get_encoding() {
            Some(source_encoding_read) => {
                let source_encoding = source_encoding_read.0.read().unwrap();
                return source_encoding.get_default_file_extension();
                /*if source_encoding.is_same_variant_in_enum_as(&*self.tmp_encoding.0.read().unwrap())
                {
                    return source_encoding.get_default_file_extension();
                } else {
                    return "downloaded".to_string();
                }*/
            }
            None => panic!("get_download_extension called against source storage without encoding"),
        }
    }
}
