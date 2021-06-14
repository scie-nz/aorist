use crate::algorithms::*;
use crate::asset::*;
use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct SingleObjectiveRegressor {
    name: String,
    #[constrainable]
    source_data: Vec<Asset>,
    #[constrainable]
    algorithm: RegressionAlgorithm,
}
