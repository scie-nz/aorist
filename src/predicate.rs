use crate::attributes::AttributeOrValue;
use crate::concept::{AoristConcept, ConceptEnum, WrappedConcept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
pub use crate::sql_parser::AttrMap;
use sqlparser::ast::{BinaryOperator, DataType, Expr};
use pyo3::exceptions::PyValueError;

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum PredicateInnerOrTerminal {
    PredicateTerminal(AttributeOrValue),
    PredicateInner(Box<PredicateInner>),
}
impl PredicateInnerOrTerminal {
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self::PredicateInner(Box::new(PredicateInner::try_from(
                x, attr,
            )?))),
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } | Expr::Value { .. } => Ok(
                Self::PredicateTerminal(AttributeOrValue::try_from(x, attr)?),
            ),
            _ => Err("Only Binary operators, identifiers or values supported as nodes".into()),
        }
    }
    pub fn as_sql(&self) -> String {
        match &self {
            PredicateInnerOrTerminal::PredicateTerminal(x) => x.as_sql(),
            PredicateInnerOrTerminal::PredicateInner(x) => format!("({})", x.as_sql()).to_string(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
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
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct PredicateInner {
    operator: Operator,
    left: PredicateInnerOrTerminal,
    right: PredicateInnerOrTerminal,
}
impl<'a> FromPyObject<'a> for Box<PredicateInner> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let inner = PredicateInner::extract(ob)?;
        Ok(Box::new(inner))
    }
}
impl PredicateInner {
    fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { left, op, right } => {
                let operator = match op {
                    BinaryOperator::Gt => Operator::Gt(">".to_string()),
                    BinaryOperator::GtEq => Operator::GtEq(">=".to_string()),
                    BinaryOperator::Eq => Operator::Eq("=".to_string()),
                    BinaryOperator::NotEq => Operator::NotEq("!=".to_string()),
                    BinaryOperator::Lt => Operator::Lt("<".to_string()),
                    BinaryOperator::LtEq => Operator::LtEq("<=".to_string()),
                    _ => return Err("Only > operators supported.".into()),
                };
                Ok(Self {
                    operator,
                    left: PredicateInnerOrTerminal::try_from(*left, attr)?,
                    right: PredicateInnerOrTerminal::try_from(*right, attr)?,
                })
            }
            _ => Err("Only binary operators supported.".into()),
        }
    }
    fn as_sql(&self) -> String {
        format!(
            "{} {} {}",
            self.left.as_sql(),
            self.operator.as_sql(),
            self.right.as_sql()
        )
        .to_string()
    }
}

#[aorist_concept]
pub struct Predicate {
    root: PredicateInner,
}

impl Predicate {
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self {
                root: PredicateInner::try_from(x, attr)?,
                constraints: Vec::new(),
                tag: None,
                uuid: None,
            }),
            _ => Err("Only binary operators supported.".into()),
        }
    }
    pub fn as_sql(&self) -> String {
        self.root.as_sql()
    }
}
impl InnerPredicate {
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self {
                root: PredicateInner::try_from(x, attr)?,
                tag: None,
            }),
            _ => Err("Only binary operators supported.".into()),
        }
    }
}

