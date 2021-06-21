// pub use crate::universe::*;
use aorist_concept::{aorist2, Constrainable};
pub use aorist_primitives::{
    register_concept, Ancestry, AoristConcept, ConceptEnum, ConceptEnumNew, TConceptEnum,
};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};
use tracing::debug;
use uuid::Uuid;
pub trait AoristConceptNew {
    type TChildrenEnum: ConceptEnumNew;
    fn get_uuid(&self) -> Option<Uuid>;
    fn get_children(
        &self,
    ) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Option<Uuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
}
pub struct AoristRef<T: PartialEq + Serialize + Debug + Clone>(Arc<RwLock<T>>);
impl<T: PartialEq + Serialize + Debug + Clone> PartialEq for AoristRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.read().unwrap().eq(&other.0.read().unwrap())
    }
}
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
#[aorist2]
pub struct GlobalPermissionsAdmin {}
#[aorist2]
pub enum Role {
    GlobalPermissionsAdmin(AoristRef<GlobalPermissionsAdmin>),
}
#[aorist2]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    pub unixname: String,
    #[constrainable2]
    roles: Option<Vec<AoristRef<Role>>>,
}
// pub struct WrappedConcept<'a, T> {
//     pub inner: T,
//     pub _phantom_lt: std::marker::PhantomData<&'a ()>,
// }
register_concept!(Concept, ConceptAncestry, GlobalPermissionsAdmin, Role, User);

// use crate::access_policy::*;
// use crate::algorithms::*;
// use crate::asset::*;
// use crate::attributes::*;
// use crate::compliance::*;
// use crate::compression::*;
// use crate::dataset::*;
// use crate::encoding::*;
// use crate::endpoints::*;
// use crate::header::*;
// use crate::layout::*;
// use crate::location::*;
// use crate::models::*;
// use crate::predicate::*;
// use crate::role::*;
// use crate::role_binding::*;
// use crate::schema::*;
// use crate::storage::*;
// use crate::storage_setup::*;
// use crate::template::*;
// use crate::user::*;
// use crate::user_group::*;
// pub struct WrappedConcept<'a, T> {
//     pub inner: T,
//     pub _phantom_lt: std::marker::PhantomData<&'a ()>,
// }
//
// register_concept!(
//     Concept,
//     ConceptAncestry,
//     AccessPolicy,
//     ApproveAccessSelector,
//     RegressionAlgorithm,
//     RandomForestRegressionAlgorithm,
//     SVMRegressionAlgorithm,
//     Asset,
//     StaticDataTable,
//     SupervisedModel,
//     DerivedAsset,
//     Attribute,
//     Predicate,
//     APILayout,
//     APIOrFileLayout,
//     FileBasedStorageLayout,
//     SingleFileLayout,
//     PushshiftSubredditPostsAPILayout,
//     TabularLayout,
//     DynamicTabularLayout,
//     StaticTabularLayout,
//     Granularity,
//     DailyGranularity,
//     DataSet,
//     Role,
//     GlobalPermissionsAdmin,
//     GzipCompression,
//     DataCompression,
//     ZipCompression,
//     ComplianceConfig,
//     CSVHeader,
//     FileHeader,
//     AlluxioLocation,
//     BigQueryLocation,
//     GCSLocation,
//     GithubLocation,
//     HiveLocation,
//     LocalFileSystemLocation,
//     OnPremiseLocation,
//     MinioLocation,
//     S3Location,
//     PostgresLocation,
//     PushshiftAPILocation,
//     RemoteLocation,
//     SQLiteLocation,
//     WebLocation,
//     Model,
//     SingleObjectiveRegressor,
//     GDBEncoding,
//     CSVEncoding,
//     TSVEncoding,
//     Encoding,
//     JSONEncoding,
//     NewlineDelimitedJSONEncoding,
//     ORCEncoding,
//     ONNXEncoding,
//     UndefinedTabularSchema,
//     TabularSchema,
//     TimeOrderedTabularSchema,
//     DataSchema,
//     Universe,
//     LocalStorageSetup,
//     RemoteStorageSetup,
//     ReplicationStorageSetup,
//     ComputedFromLocalData,
//     StorageSetup,
//     Storage,
//     BigQueryStorage,
//     SQLiteStorage,
//     HiveTableStorage,
//     RemoteStorage,
//     LocalFileStorage,
//     PostgresStorage,
//     GitStorage,
//     RoleBinding,
//     DatumTemplate,
//     IdentifierTuple,
//     RowStruct,
//     IntegerMeasure,
//     Filter,
//     User,
//     UserGroup,
//     EndpointConfig,
//     AWSConfig,
//     GCPConfig,
//     GiteaConfig,
//     PostgresConfig,
//     AlluxioConfig,
//     RangerConfig,
//     PrestoConfig,
//     MinioConfig,
//     TrainedFloatMeasure,
//     PredictionsFromTrainedFloatMeasure
// );
