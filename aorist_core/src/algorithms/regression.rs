use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct RandomForestRegressionAlgorithm {}
#[aorist]
pub struct SVMRegressionAlgorithm {}

#[aorist]
pub enum RegressionAlgorithm {
    RandomForestRegressionAlgorithm(RandomForestRegressionAlgorithm),
    SVMRegressionAlgorithm(SVMRegressionAlgorithm),
}
