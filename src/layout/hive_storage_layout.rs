#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;
use derivative::Derivative;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct StaticHiveTableLayout {
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq="ignore", Debug="ignore")]
    constraints: Vec<Rc<Constraint>>,
}

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct DailyGranularity {
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq="ignore", Debug="ignore")]
    constraints: Vec<Rc<Constraint>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(DailyGranularity),
}

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct DynamicHiveTableLayout {
    #[constrainable]
    granularity: Granularity,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq="ignore", Debug="ignore")]
    constraints: Vec<Rc<Constraint>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
    DynamicHiveTableLayout(DynamicHiveTableLayout),
}
