use aorist_primitives::AVec;
use crate::algorithms::*;
use crate::asset::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AString;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct SingleObjectiveRegressor {
    name: AString,
    #[constrainable]
    source_data: AVec<AoristRef<Asset>>,
    #[constrainable]
    algorithm: AoristRef<RegressionAlgorithm>,
}
