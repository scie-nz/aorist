use crate::algorithms::*;
use crate::asset::*;
use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct SingleObjectiveRegressor {
    name: String,
    #[constrainable]
    source_data: Vec<AoristRef<Asset>>,
    #[constrainable]
    algorithm: AoristRef<RegressionAlgorithm>,
}
