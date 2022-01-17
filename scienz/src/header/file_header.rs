use crate::header::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
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
