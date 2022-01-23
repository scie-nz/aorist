#[cfg(feature = "sql")]
use crate::attributes::AttrMap;
use crate::attributes::AttributeOrValue;
use abi_stable::std_types::ROption;
use abi_stable::StableAbi;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "sql")]
use sqlparser::ast::{BinaryOperator, Expr};
use std::fmt::Debug;

#[repr(C)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum Operator {
    GtEq(AString),
    Gt(AString),
    Eq(AString),
    NotEq(AString),
    Lt(AString),
    LtEq(AString),
}
impl Operator {
    pub fn as_sql(&self) -> String {
        match &self {
            Operator::GtEq(_) => ">=".into(),
            Operator::Gt(_) => ">".into(),
            Operator::Eq(_) => "=".into(),
            Operator::NotEq(_) => "!=".into(),
            Operator::Lt(_) => "<".into(),
            Operator::LtEq(_) => "<=".into(),
        }
    }
}
#[repr(C)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum PredicateInnerOrTerminal {
    PredicateTerminal(AoristRef<AttributeOrValue>),
    PredicateInner(AoristRef<PredicateInner>),
}
impl PredicateInnerOrTerminal {
    #[cfg(feature = "sql")]
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, AString> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self::PredicateInner(AoristRef(RArc::new(RRwLock::new(
                PredicateInner::try_from(x, attr)?,
            ))))),
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } | Expr::Value { .. } => {
                Ok(Self::PredicateInner(AoristRef(RArc::new(RRwLock::new(
                    AttributeOrValue::try_from(x, attr)?,
                )))))
            }
            _ => Err("Only Binary operators, identifiers or values supported as nodes".into()),
        }
    }
    pub fn as_sql(&self) -> AString {
        match &self {
            PredicateInnerOrTerminal::PredicateTerminal(x) => x.0.read().as_sql(),
            PredicateInnerOrTerminal::PredicateInner(x) => {
                format!("({})", x.0.read().as_sql()).as_str().into()
            }
        }
    }
}
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
pub struct PredicateInner {
    operator: Operator,
    left: PredicateInnerOrTerminal,
    right: PredicateInnerOrTerminal,
}
/*#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for AoristRef<PredicateInner> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let inner = PredicateInner::extract(ob)?;
        Ok(AoristRef(RArc::new(RRwLock::new(inner))))
    }
}*/
impl PredicateInner {
    #[cfg(feature = "sql")]
    fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, AString> {
        match x {
            Expr::BinaryOp { left, op, right } => {
                let operator = match op {
                    BinaryOperator::Gt => Operator::Gt(">".into()),
                    BinaryOperator::GtEq => Operator::GtEq(">=".into()),
                    BinaryOperator::Eq => Operator::Eq("=".into()),
                    BinaryOperator::NotEq => Operator::NotEq("!=".into()),
                    BinaryOperator::Lt => Operator::Lt("<".into()),
                    BinaryOperator::LtEq => Operator::LtEq("<=".into()),
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

#[aorist]
pub struct Predicate {
    root: PredicateInner,
}

impl Predicate {
    #[cfg(feature = "sql")]
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, AString> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self {
                root: PredicateInner::try_from(x, attr)?,
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
