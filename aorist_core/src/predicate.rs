use crate::concept::{AoristConcept, ConceptEnum};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[cfg(feature = "sql")]
use sqlparser::ast::{BinaryOperator, Expr};

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum Operator {
    GtEq(String),
    Gt(String),
    Eq(String),
    NotEq(String),
    Lt(String),
    LtEq(String),
}
impl Operator {
    pub fn as_sql(&self) -> String {
        match &self {
            Operator::GtEq(_) => ">=".to_string(),
            Operator::Gt(_) => ">".to_string(),
            Operator::Eq(_) => "=".to_string(),
            Operator::NotEq(_) => "!=".to_string(),
            Operator::Lt(_) => "<".to_string(),
            Operator::LtEq(_) => "<=".to_string(),
        }
    }
}
