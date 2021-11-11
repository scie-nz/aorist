use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::adjacent_polygons_schema::*;
use crate::schema::gdal_fillnodata_schema::*;
use crate::schema::linz_primary_parcels_schema::*;
use crate::schema::linz_property_titles_schema::*;
use crate::schema::normalized_point_cloud_schema::*;
use crate::schema::labeled_point_cloud_schema::*;
use crate::schema::naip_metadata_schema::*;
use crate::schema::point_cloud_boundary_schema::*;
use crate::schema::point_cloud_bounding_box_schema::*;
use crate::schema::point_cloud_info_schema::*;
use crate::schema::point_cloud_metadata_schema::*;
use crate::schema::point_cloud_schema::*;
use crate::schema::point_cloud_transformation_schema::*;
use crate::schema::point_cloud_subset_schema::*;
use crate::schema::point_cloud_raster_difference_schema::*;
use crate::schema::polygon_intersection_schema::*;
use crate::schema::polygon_schema::*;
use crate::schema::polygon_from_raster_schema::*;
use crate::schema::raster_schema::*;
use crate::schema::raster_difference_schema::*;
use crate::schema::raster_from_point_cloud_schema::*;
use crate::schema::raster_polygon_intersection_schema::*;
use crate::schema::tao_marker_controlled_watershed_segmentation_schema::*;
use crate::schema::tao_watershed_segmentation_schema::*;
use crate::schema::tao_li_segmentation_schema::*;
use crate::schema::tao_da_silva_segmentation_schema::*;
use crate::schema::tao_crown_hull_filter_schema::*;
use crate::schema::tao_crown_hull_schema::*;
use crate::schema::tao_to_eco_object_schema::*;
use crate::schema::tree_detection_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::schema_enum;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
#[cfg(feature = "python")]
use pyo3::prelude::*;

schema_enum! {
    name: GeospatialAssetSchema
    variants:
    - AdjacentPolygonsSchema
    - GDALFillNoDataSchema
    - LINZPrimaryParcelsSchema
    - LINZPropertyTitlesSchema
    - LabeledPointCloudSchema
    - NAIPMetadataSchema
    - NormalizedPointCloudSchema
    - PointCloudSchema
    - PointCloudBoundarySchema
    - PointCloudBoundingBoxSchema
    - PointCloudInfoSchema
    - PointCloudMetadataSchema
    - PointCloudRasterDifferenceSchema
    - PointCloudSubsetSchema
    - PointCloudTransformationSchema
    - RasterSchema
    - RasterDifferenceSchema
    - RasterPolygonIntersectionSchema
    - RasterFromPointCloudSchema
    - PolygonFromRasterSchema
    - PolygonIntersectionSchema
    - PolygonSchema
    - TAOCrownHullSchema
    - TAOCrownHullFilterSchema
    - TAOMarkerControlledWatershedSegmentationSchema
    - TAOWatershedSegmentationSchema
    - TAOLiSegmentationSchema
    - TAODaSilvaSegmentationSchema
    - TAOToEcoObjectSchema
    - TreeDetectionSchema
}
