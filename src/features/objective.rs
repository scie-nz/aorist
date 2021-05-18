#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren,  InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct ContinuousObjective {}

#[aorist_concept]
pub enum ContinuousRegressionObjective {
    ContinuousObjective(ContinuousObjective),
}

#[aorist_concept]
pub enum RegressionObjective {
    ContinuousRegressionObjective(ContinuousRegressionObjective),
}

#[aorist_concept]
pub struct Foo {
    bar: SomeRef,
}

#[aorist_concept]
pub enum Bar {
    Foo(Foo),
}
impl Eq for Bar {}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SomeRef {
    bar: Box<Bar>,
}
