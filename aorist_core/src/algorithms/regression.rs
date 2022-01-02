use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum, WrappedConcept};
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct RandomForestRegressionAlgorithm {}
#[aorist]
pub struct SVMRegressionAlgorithm {}

#[aorist]
pub enum RegressionAlgorithm {
    RandomForestRegressionAlgorithm(AoristRef<RandomForestRegressionAlgorithm>),
    SVMRegressionAlgorithm(AoristRef<SVMRegressionAlgorithm>),
}
