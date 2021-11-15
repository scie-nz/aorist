pub use crate::universe::*;
pub use aorist_primitives::{
    register_concept, Ancestry, AoristConcept, AoristUniverse, ConceptEnum, TConceptEnum,
};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use tracing::debug;
use uuid::Uuid;

pub struct AoristRef<T: PartialEq + Serialize + Debug + Clone>(pub Arc<RwLock<T>>);

#[derive(Clone)]
pub struct RefABI<T: PartialEq + Serialize + Debug + Clone>(
    pub abi_stable::std_types::RArc<abi_stable::external_types::parking_lot::rw_lock::RRwLock<T>>,
);

#[cfg(feature = "python")]
impl<'a, T: PartialEq + Serialize + Debug + Clone + FromPyObject<'a>> FromPyObject<'a>
    for AoristRef<T>
{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        Ok(AoristRef(Arc::new(RwLock::new(T::extract(ob)?))))
    }
}

impl<T: PartialEq + Eq + Serialize + Debug + Clone> PartialEq for AoristRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.read().unwrap().eq(&other.0.read().unwrap())
    }
}
impl<T: PartialEq + Eq + Serialize + Debug + Clone> Eq for AoristRef<T> {}
impl<T: PartialEq + Serialize + Debug + Clone> Serialize for AoristRef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.read().unwrap().serialize(serializer)
    }
}
impl<'de, T: Deserialize<'de> + PartialEq + Serialize + Debug + Clone> Deserialize<'de>
    for AoristRef<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let d = T::deserialize(deserializer)?;
        Ok(Self(Arc::new(RwLock::new(d))))
    }
}
impl<T: Clone + Debug + Serialize + PartialEq> Clone for AoristRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Debug + Clone + Serialize + PartialEq> Debug for AoristRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.read().unwrap().fmt(f)
    }
}
pub struct WrappedConcept<T>
where
    T: Debug + Clone + Serialize + PartialEq,
{
    pub inner: AoristRef<T>,
}
impl<T: Debug + Clone + Serialize + PartialEq> Hash for AoristRef<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.read().unwrap().hash(state);
    }
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
    PolygonAsset
);
