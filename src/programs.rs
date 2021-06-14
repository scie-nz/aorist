pub use crate::sql_parser::AttrMap;
use std::sync::Arc;
use uuid::Uuid;
use crate::constraint::Constraint;
use aorist_attributes::TAttribute;

pub trait TValue {}
include!(concat!(env!("OUT_DIR"), "/programs.rs"));
