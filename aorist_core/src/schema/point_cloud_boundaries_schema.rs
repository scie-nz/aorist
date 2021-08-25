use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::derived_asset_schema::DerivedAssetSchema;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

#[aorist]
pub struct PointCloudBoundariesSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub source: AoristRef<StaticDataTable>,
}
impl PointCloudBoundariesSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.datum_template.0.read().unwrap().get_attributes()
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
impl DerivedAssetSchema<'_> for PointCloudBoundariesSchema {
    type SourceAssetType = StaticDataTable; 
    fn get_source(&self) -> AoristRef<Self::SourceAssetType> {
        self.source.clone()
    }
}
