use aorist_concept::{aorist, Constrainable};
use crate::{AoristConcept, ConceptEnum};
use crate::header::CSVHeader;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

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
