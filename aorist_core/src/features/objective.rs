#![allow(non_snake_case)]
use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct ContinuousObjective {}

#[aorist]
pub enum ContinuousRegressionObjective {
    ContinuousObjective(ContinuousObjective),
}

#[aorist]
pub enum RegressionObjective {
    ContinuousRegressionObjective(ContinuousRegressionObjective),
}

#[aorist]
pub struct Foo {
    bar: SomeRef,
}

#[aorist]
pub enum Bar {
    Foo(Foo),
}
impl Eq for Bar {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SomeRef {
    bar: Box<Bar>,
}
