use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::attribute;
use aorist_attributes::*;
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

#[aorist]
pub struct PointCloudInfoSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub source: AoristRef<PointCloudAsset>,
}
impl PointCloudInfoSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        vec![
            attribute! { FreeText(
                "prefix".to_string(), 
                Some("File Prefix".to_string()), 
                false
            )}, 
            attribute! { FreeText(
                "json".to_string(), 
                Some("JSON for pdal info".to_string()), 
                false
            )}, 
        ]
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
