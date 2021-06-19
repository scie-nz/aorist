use crate::header::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
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
