use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::template::*;
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
pub struct TabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub attributes: AVec<AString>,
}
impl TabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
