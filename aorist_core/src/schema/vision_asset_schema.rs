
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::flann_knn_match_schema::*;
use crate::schema::homography_from_knn_match_schema::*;
use crate::schema::image_from_raster_schema::*;
use crate::schema::perspective_transform_from_homography_schema::*;
use crate::schema::pytorch_image_collection_mlp_schema::*;
use crate::schema::sift_affine_image_key_point_schema::*;
use crate::schema::transform_image_corpus_through_mlp_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum VisionAssetSchema {
    #[constrainable]
    ImageFromRasterSchema(AoristRef<ImageFromRasterSchema>),
    #[constrainable]
    PyTorchImageCollectionMLPSchema(AoristRef<PyTorchImageCollectionMLPSchema>),
    #[constrainable]
    TransformImageCorpusThroughMLPSchema(AoristRef<TransformImageCorpusThroughMLPSchema>),
    #[constrainable]
    SIFTAffineImageKeyPointSchema(AoristRef<SIFTAffineImageKeyPointSchema>),
    #[constrainable]
    FLANNKNNMatchSchema(AoristRef<FLANNKNNMatchSchema>),
    #[constrainable]
    HomographyFromKNNMatchSchema(AoristRef<HomographyFromKNNMatchSchema>),
    #[constrainable]
    PerspectiveTransformFromHomographySchema(AoristRef<PerspectiveTransformFromHomographySchema>),
}

impl VisionAssetSchema {
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        match self {
            Self::ImageFromRasterSchema(x) => x.0.read().get_attributes(),
            Self::PyTorchImageCollectionMLPSchema(x) => x.0.read().get_attributes(),
            Self::TransformImageCorpusThroughMLPSchema(x) => x.0.read().get_attributes(),
            Self::SIFTAffineImageKeyPointSchema(x) => x.0.read().get_attributes(),
            Self::FLANNKNNMatchSchema(x) => x.0.read().get_attributes(),
            Self::HomographyFromKNNMatchSchema(x) => x.0.read().get_attributes(),
            Self::PerspectiveTransformFromHomographySchema(x) => x.0.read().get_attributes(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            VisionAssetSchema::ImageFromRasterSchema(x) => x.0.read().get_datum_template(),
            VisionAssetSchema::PyTorchImageCollectionMLPSchema(x) => {
                x.0.read().get_datum_template()
            }
            VisionAssetSchema::TransformImageCorpusThroughMLPSchema(x) => {
                x.0.read().get_datum_template()
            }
            VisionAssetSchema::SIFTAffineImageKeyPointSchema(x) => x.0.read().get_datum_template(),
            VisionAssetSchema::FLANNKNNMatchSchema(x) => x.0.read().get_datum_template(),
            VisionAssetSchema::HomographyFromKNNMatchSchema(x) => x.0.read().get_datum_template(),
            VisionAssetSchema::PerspectiveTransformFromHomographySchema(x) => {
                x.0.read().get_datum_template()
            }
        }
    }
}
