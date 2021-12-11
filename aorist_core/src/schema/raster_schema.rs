use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct RasterSchema {
    pub datum_template: AoristRef<DatumTemplate>,
}
impl RasterSchema {
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.datum_template.0.read().get_attributes()
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
