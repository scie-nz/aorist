use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::header::UpperSnakeCaseCSVHeader;


#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}
