use crate::concept::{AoristConcept, Concept};
use crate::constraint::AoristConstraint;
use crate::constraint::Constraint;
use crate::header::UpperSnakeCaseCSVHeader;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
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
