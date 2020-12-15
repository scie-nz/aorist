use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::header::UpperSnakeCaseCSVHeader;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}
