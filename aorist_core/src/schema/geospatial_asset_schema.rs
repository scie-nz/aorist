use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::raster_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum GeospatialAssetSchema {
    RasterSchema(AoristRef<RasterSchema>),
}
