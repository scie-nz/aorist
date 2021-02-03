use crate::concept::{AoristConcept, Concept};
use crate::constraint::AoristConstraint;
use crate::constraint::Constraint;
use crate::header::UpperSnakeCaseCSVHeader;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
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
