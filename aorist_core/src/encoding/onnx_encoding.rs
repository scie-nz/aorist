use aorist_concept::{aorist, Constrainable};
use crate::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist]
pub struct ONNXEncoding {}
