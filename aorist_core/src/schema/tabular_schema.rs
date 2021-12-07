use aorist_primitives::AVec;
#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AString;
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
