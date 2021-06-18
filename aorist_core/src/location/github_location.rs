use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct GithubLocation {
    pub organization: String,
    pub repository: String,
    pub path: String,
    pub branch: String,
}
