use aorist_constraint::Constraint;
use aorist_core::{Concept};
use aorist_attributes::TAttribute;
use aorist_core::AttrMap;
use std::sync::Arc;
use uuid::Uuid;
use pyo3::create_exception;

pub trait TValue {}
include!(concat!(env!("OUT_DIR"), "/programs.rs"));
