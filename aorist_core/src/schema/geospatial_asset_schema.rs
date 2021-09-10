use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::linz_primary_parcels_schema::*;
use crate::schema::linz_property_titles_schema::*;
use crate::schema::point_cloud_boundary_schema::*;
use crate::schema::point_cloud_bounding_box_schema::*;
use crate::schema::point_cloud_info_schema::*;
use crate::schema::point_cloud_metadata_schema::*;
use crate::schema::point_cloud_schema::*;
use crate::schema::polygon_intersection_schema::*;
use crate::schema::polygon_schema::*;
use crate::schema::raster_schema::*;
use crate::template::*;
use aorist_primitives::schema_enum;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;

schema_enum! {
    name: GeospatialAssetSchema
    variants:
    - LINZPrimaryParcelsSchema
    - LINZPropertyTitlesSchema
    - PointCloudSchema
    - PointCloudBoundarySchema
    - PointCloudBoundingBoxSchema
    - PointCloudInfoSchema
    - PointCloudMetadataSchema
    - RasterSchema
    - PolygonIntersectionSchema
    - PolygonSchema
}
