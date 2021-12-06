use crate::algorithms::*;
use crate::asset::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::AString;
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct SingleObjectiveRegressor {
    name: AString,
    #[constrainable]
    source_data: Vec<AoristRef<Asset>>,
    #[constrainable]
    algorithm: AoristRef<RegressionAlgorithm>,
}
