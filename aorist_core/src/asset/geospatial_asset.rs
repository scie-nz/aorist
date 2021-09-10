use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, asset_enum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use std::sync::{Arc, RwLock};

asset_enum! {
    name: GeospatialAsset
    variants:
    - RasterAsset
    - PointCloudAsset
    - PolygonCollectionAsset
    - PointCloudInfoAsset
    - PointCloudMetadataAsset
    - PolygonIntersectionAsset
}
