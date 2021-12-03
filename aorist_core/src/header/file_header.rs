#![allow(unused_parens)]
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum FileHeader {
    CSVHeader(AoristRef<CSVHeader>),
}

impl FileHeader {
    pub fn get_num_lines(&self) -> usize {
        let FileHeader::CSVHeader(x) = self;
        let read = x.0.read();
        match read.num_lines {
            None => 1,
            Some(n) => n,
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
