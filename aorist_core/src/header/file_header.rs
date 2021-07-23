#![allow(unused_parens)]
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum FileHeader {
    CSVHeader(AoristRef<CSVHeader>),
}

impl FileHeader {
    #[allow(dead_code)]
    pub fn get_num_lines(&self) -> usize {
        let FileHeader::CSVHeader(x) = self;
        let read = x.0.read().unwrap();
        match read.num_lines {
            None => 1,
            Some(n) => n,
        }
    }
}
