pub use aorist_primitives::{register_concept, AoristConcept, ConceptEnum, TConceptEnum};
pub use crate::universe::*;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::convert::TryFrom;
use tracing::debug;
        
use crate::access_policy::*;
use crate::algorithms::*;
use crate::asset::*;
use crate::attributes::*; 
use crate::predicate::*;
use crate::layout::*;
use crate::dataset::*; 
use crate::role::*; 
use crate::compression::*;
use crate::compliance::*;
use crate::header::*;
use crate::location::*;
use crate::models::*;
use crate::encoding::*; 
use crate::schema::*; 
use crate::storage_setup::*; 
use crate::storage::*; 
use crate::role_binding::*; 
use crate::template::*; 
use crate::user::*; 
use crate::user_group::*;
use crate::endpoints::*; 

pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a>;
}
pub struct WrappedConcept<'a, T> {
    pub inner: T,
    pub _phantom_lt: std::marker::PhantomData<&'a ()>,
}

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
    SupervisedModel, 
    DerivedAsset, 
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
    AWSConfig, 
    GCPConfig, 
    GiteaConfig, 
    PostgresConfig, 
    AlluxioConfig, 
    RangerConfig, 
    PrestoConfig, 
    MinioConfig, 
    TrainedFloatMeasure, 
    PredictionsFromTrainedFloatMeasure 
);
