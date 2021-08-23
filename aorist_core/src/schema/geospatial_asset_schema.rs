use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::raster_schema::*;
use crate::schema::point_cloud_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

#[aorist]
pub enum GeospatialAssetSchema {
    RasterSchema(AoristRef<RasterSchema>),
    PointCloudSchema(AoristRef<PointCloudSchema>),
}
impl GeospatialAssetSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        match self {
            Self::RasterSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PointCloudSchema(x) => x.0.read().unwrap().get_attributes(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            Self::RasterSchema(x) => x.0.read().unwrap().get_datum_template(),
            Self::PointCloudSchema(x) => x.0.read().unwrap().get_datum_template()
        }
    }
}
