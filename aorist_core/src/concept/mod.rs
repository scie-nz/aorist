pub use crate::universe::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption};
pub use aorist_primitives::{
    register_concept, AOption, AString, AVec, Ancestry, AoristConcept, AoristConceptBase,
    AoristRef, AoristUniverse, AoristUniverseBase, ConceptEnum, ToplineConcept, ToplineConceptBase,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct RefABI<T: PartialEq + Serialize + Debug + Clone>(
    pub abi_stable::std_types::RArc<abi_stable::external_types::parking_lot::rw_lock::RRwLock<T>>,
);

pub struct WrappedConcept<T>
where
    T: Debug + Clone + Serialize + PartialEq,
{
    pub inner: AoristRef<T>,
}

use crate::access_policy::*;
use crate::algorithms::*;
use crate::asset::*;
use crate::attributes::*;
use crate::compliance::*;
use crate::compression::*;
use crate::dataset::*;
use crate::encoding::*;
use crate::endpoints::*;
use crate::header::*;
use crate::layout::*;
use crate::location::*;
use crate::models::*;
use crate::predicate::*;
use crate::role::*;
use crate::role_binding::*;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use crate::template::*;
use crate::user::*;
use crate::user_group::*;
register_concept!(
    Concept,
    ConceptAncestry,
    AccessPolicy,
    ApproveAccessSelector,
    RegressionAlgorithm,
    RandomForestRegressionAlgorithm,
    SVMRegressionAlgorithm,
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
    Model,
    SingleObjectiveRegressor,
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
    ComputedFromLocalData,
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
