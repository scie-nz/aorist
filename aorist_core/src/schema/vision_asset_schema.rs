use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::pytorch_image_collection_mlp_schema::*;
use crate::schema::image_from_raster_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum VisionAssetSchema {
    #[constrainable]
    ImageFromRasterSchema(AoristRef<ImageFromRasterSchema>),
    #[constrainable]
    PyTorchImageCollectionMLPSchema(AoristRef<PyTorchImageCollectionMLPSchema>),
}

impl VisionAssetSchema {
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        match self {
            Self::ImageFromRasterSchema(x) => x.0.read().unwrap().get_attributes(),
            Self::PyTorchImageCollectionMLPSchema(x) => x.0.read().unwrap().get_attributes(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            VisionAssetSchema::ImageFromRasterSchema(x) => {
                x.0.read().unwrap().get_datum_template()
            }
            VisionAssetSchema::PyTorchImageCollectionMLPSchema(x) => {
                x.0.read().unwrap().get_datum_template()
            }
        }
    }
}
