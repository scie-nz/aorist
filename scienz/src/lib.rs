#![allow(unused_parens)]
mod access_policy;
mod asset;
mod attributes;
mod compliance;
mod compression;
mod dataset;
mod encoding;
mod endpoints;
mod error;
mod header;
mod layout;
mod location;
mod predicate;
mod role;
mod role_binding;
mod schema;
mod storage;
mod storage_setup;
mod template;
mod universe;
mod user;
mod user_group;

pub use access_policy::*;
pub use asset::*;
pub use attributes::*;
pub use compliance::*;
pub use compression::*;
pub use dataset::*;
pub use encoding::*;
pub use endpoints::*;
pub use error::*;
pub use header::*;
pub use layout::*;
pub use location::*;
pub use predicate::*;
pub use role::*;
pub use role_binding::*;
pub use schema::*;
pub use storage::*;
pub use storage_setup::*;
pub use template::*;
pub use universe::*;
pub use user::*;
pub use user_group::*;

use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption};
pub use aorist_primitives::{
    register_concept, AConcept, Ancestry, AoristConceptBase, AoristUniverse,
    AoristUniverseBase, ConceptEnum, ToplineConcept, ToplineConceptBase,
};
use aorist_util::AUuid;
use aorist_util::{AOption, AString, AVec, AoristRef};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::debug;
register_concept!(
    Concept,
    ConceptAncestry,
    AccessPolicy,
    ApproveAccessSelector,
    Asset,
    StaticDataTable,
    Attribute,
    Predicate,
    APILayout,
    APIOrFileLayout,
    FileBasedStorageLayout,
    SingleFileLayout,
    PushshiftSubredditPostsAPILayout,
    TabularLayout,
    DynamicTabularLayout,
    StaticTabularLayout,
    Granularity,
    DailyGranularity,
    DataSet,
    Role,
    GlobalPermissionsAdmin,
    GzipCompression,
    DataCompression,
    ZipCompression,
    ComplianceConfig,
    CSVHeader,
    FileHeader,
    AlluxioLocation,
    BigQueryLocation,
    GCSLocation,
    GithubLocation,
    HiveLocation,
    LocalFileSystemLocation,
    OnPremiseLocation,
    MinioLocation,
    S3Location,
    PostgresLocation,
    PushshiftAPILocation,
    RemoteLocation,
    SQLiteLocation,
    WebLocation,
    GDBEncoding,
    CSVEncoding,
    TSVEncoding,
    Encoding,
    JSONEncoding,
    NewlineDelimitedJSONEncoding,
    ORCEncoding,
    ONNXEncoding,
    UndefinedTabularSchema,
    TabularSchema,
    TimeOrderedTabularSchema,
    DataSchema,
    Universe,
    LocalStorageSetup,
    RemoteStorageSetup,
    ReplicationStorageSetup,
    StorageSetup,
    Storage,
    BigQueryStorage,
    SQLiteStorage,
    HiveTableStorage,
    RemoteStorage,
    LocalFileStorage,
    PostgresStorage,
    GitStorage,
    RoleBinding,
    DatumTemplate,
    IdentifierTuple,
    RowStruct,
    IntegerMeasure,
    Filter,
    User,
    UserGroup,
    EndpointConfig,
    TrainedFloatMeasure,
    PredictionsFromTrainedFloatMeasure,
    FasttextEmbeddingSchema,
    FasttextEmbedding,
    LongTabularSchema,
    TextCorpusSchema,
    SQLiteEncoding,
    NamedEntities,
    NamedEntitySchema,
    LanguageAsset,
    LanguageAssetSchema,
    GeoTiffEncoding,
    CompressedFileCollectionLayout,
    GeospatialAssetSchema,
    RasterSchema,
    Tensor,
    RasterAsset,
    PointCloud,
    GeospatialAsset,
    PointCloudSchema,
    PointCloudAsset,
    LASEncoding,
    LAZCompression,
    PointCloudInfo,
    PointCloudInfoSchema,
    PointCloudMetadataSchema,
    PointCloudBoundarySchema,
    DirectoryLayout,
    PolygonCollectionAsset,
    LINZPrimaryParcelsSchema,
    LINZPropertyTitlesSchema,
    TwoTierStorageSetup,
    TabularCollectionSchema,
    Polygon,
    InlineBlobStorage,
    WKTEncoding,
    PolygonSchema,
    PointCloudInfoAsset,
    PointCloudMetadataAsset,
    TextCorpus,
    SpaCyNamedEntitySchema,
    Text,
    PointCloudBoundingBoxSchema,
    PolygonIntersection,
    PolygonIntersectionSchema,
    PolygonIntersectionAsset,
    PointCloudSubsetSchema,
    RasterFromPointCloudSchema,
    RasterDifferenceSchema,
    PolygonFromRasterSchema,
    Raster,
    NormalizedPointCloudSchema,
    LabeledPointCloudSchema,
    TAODaSilvaSegmentationSchema,
    TAOLiSegmentationSchema,
    TAOWatershedSegmentationSchema,
    PointCloudTransformationSchema,
    PointCloudRasterDifferenceSchema,
    TAOCrownHullSchema,
    TreeDetectionSchema,
    ShapefileEncoding,
    GDALFillNoDataSchema,
    TAOMarkerControlledWatershedSegmentationSchema,
    NAIPMetadataSchema,
    S3Storage,
    XMLEncoding,
    BZip2Compression,
    KMLEncoding,
    AdjacentPolygonsSchema,
    GraphAsset,
    SimpleUndirectedGraphAsset,
    TAOCrownHullFilterSchema,
    TAOToEcoObjectSchema,
    GraphAssetSchema,
    EdgelistFilterSchema,
    RasterPolygonIntersectionSchema,
    GPKGEncoding,
    ImageFromRasterSchema,
    VisionAssetSchema,
    TiffEncoding,
    ImageCorpus,
    VisionAsset,
    PyTorchImageCollectionMLPSchema,
    MLPAsset,
    TransformImageCorpusThroughMLPSchema,
    SIFTAffineImageKeyPointSchema,
    ImageCorpusKeyPoints,
    FLANNKNNMatchSchema,
    ImageCorporaKNNMatch,
    HomographyFromKNNMatchSchema,
    ImageCorporaHomography,
    PerspectiveTransformFromHomographySchema,
    GridPerspectiveTransformAsset,
    PerspectiveTransformedGriddedPolygonCollectionSchema,
    PolygonUnionSchema,
    PolygonAsset,
    PolygonCollectionWithinPolygonSchema,
    PolygonCollectionZonalStatsSchema,
    NDVISchema,
    NBRSchema,
    PolygonCollectionStatsUnionSchema,
    PolygonCollectionStatsFilterSchema
);
