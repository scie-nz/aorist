#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct StaticHiveTableLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct DailyGranularity {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(DailyGranularity),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct DynamicHiveTableLayout {
    #[constrainable]
    granularity: Granularity,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
    DynamicHiveTableLayout(DynamicHiveTableLayout),
}
