use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ReplicationStorageSetup {
    #[constrainable]
    pub source: AoristRef<Storage>,
    #[constrainable]
    pub targets: Vec<AoristRef<Storage>>,
    pub tmp_dir: AString,
    #[constrainable]
    pub tmp_encoding: AoristRef<Encoding>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyReplicationStorageSetup {
    #[getter]
    pub fn download_extension(&self) -> PyResult<String> {
        let rss = &*self.inner.0.read();
        Ok(rss.get_download_extension().as_str().into())
    }
}

impl ReplicationStorageSetup {
    pub fn get_download_extension(&self) -> AString {
        match self.source.0.read().get_encoding() {
            Some(source_encoding_read) => {
                let source_encoding = source_encoding_read.0.read();
                return source_encoding.get_default_file_extension();
                /*if source_encoding.is_same_variant_in_enum_as(&*self.tmp_encoding.0.read())
                {
                    return source_encoding.get_default_file_extension();
                } else {
                    return "downloaded".into();
                }*/
            }
            None => panic!("get_download_extension called against source storage without encoding"),
        }
    }
}
