pub use crate::sql_parser::AttrMap;
use indoc::formatdoc;
use num::Float;
use sqlparser::ast::{BinaryOperator, ColumnDef, DataType, Expr, Ident, Value};
use std::sync::Arc;
use uuid::Uuid;
use crate::constraint::Constraint;
use aorist_core::ConceptEnum;

pub trait TValue {}
include!(concat!(env!("OUT_DIR"), "/programs.rs"));
