#![allow(non_snake_case)]
use crate::constraint::Constraint;
use serde::{Deserialize, Serialize};
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum FileBasedStorageLayout {
    SingleFileLayout(SingleFileLayout),
}
