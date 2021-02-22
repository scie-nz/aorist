use crate::concept::{AoristConcept, Concept};
use crate::header::upper_snake_case_csv_header::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}

impl FileHeader {
    #[allow(dead_code)]
    pub fn get_num_lines(&self) -> usize {
        let FileHeader::UpperSnakeCaseCSVHeader(x) = self;
        match x.num_lines {
            None => 1,
            Some(n) => n,
        }
    }
}
