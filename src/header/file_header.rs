use crate::concept::{AoristConcept, ConceptEnum};
use crate::header::csv_header::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum FileHeader {
    CSVHeader(CSVHeader),
}

impl FileHeader {
    #[allow(dead_code)]
    pub fn get_num_lines(&self) -> usize {
        let FileHeader::CSVHeader(x) = self;
        match x.num_lines {
            None => 1,
            Some(n) => n,
        }
    }
}
