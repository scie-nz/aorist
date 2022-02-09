use crate::header::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub enum FileHeader {
    CSVHeader(AoristRef<CSVHeader>),
}

impl FileHeader {
    pub fn get_num_lines(&self) -> usize {
        let FileHeader::CSVHeader(x) = self;
        let read = x.0.read();
        match read.num_lines {
            AOption(ROption::RNone) => 1,
            AOption(ROption::RSome(n)) => n,
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyFileHeader {
    #[getter]
    pub fn get_num_lines(&self) -> usize {
        self.inner.0.read().get_num_lines()
    }
}
