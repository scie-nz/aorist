use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

#[aorist]
pub struct PointCloudSchema {
    pub datum_template: AoristRef<DatumTemplate>,
}
impl PointCloudSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.datum_template.0.read().unwrap().get_attributes()
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
