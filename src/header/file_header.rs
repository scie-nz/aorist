use crate::header::UpperSnakeCaseCSVHeader;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}
