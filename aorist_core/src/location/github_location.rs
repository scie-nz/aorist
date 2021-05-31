use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist]
pub struct GithubLocation {
    pub organization: String,
    pub repository: String,
    pub path: String,
    pub branch: String,
}
