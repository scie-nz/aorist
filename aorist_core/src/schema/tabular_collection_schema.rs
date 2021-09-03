#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TabularCollectionSchema {
    // same datum_template as a TabularSchema
    pub datum_template: AoristRef<DatumTemplate>,
    pub source_assets: Vec<AoristRef<Asset>>,
    pub attributes: Vec<String>,
}
impl TabularCollectionSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
