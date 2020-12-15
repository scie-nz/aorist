#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum FileBasedStorageLayout {
    SingleFileLayout(SingleFileLayout),
}
