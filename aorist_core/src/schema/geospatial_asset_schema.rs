use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::linz_primary_parcels_schema::*;
use crate::schema::linz_property_titles_schema::*;
use crate::schema::point_cloud_boundary_schema::*;
use crate::schema::point_cloud_bounding_box_schema::*;
use crate::schema::point_cloud_info_schema::*;
use crate::schema::point_cloud_metadata_schema::*;
use crate::schema::point_cloud_schema::*;
use crate::schema::point_cloud_subset_schema::*;
use crate::schema::polygon_intersection_schema::*;
use crate::schema::polygon_schema::*;
use crate::schema::polygon_from_raster_schema::*;
use crate::schema::raster_schema::*;
use crate::schema::raster_difference_schema::*;
use crate::schema::raster_from_point_cloud_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::schema_enum;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

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
    - PointCloudSubsetSchema
    - RasterSchema
    - RasterDifferenceSchema
    - RasterFromPointCloudSchema
    - PolygonFromRasterSchema
    - PolygonIntersectionSchema
    - PolygonSchema
}
