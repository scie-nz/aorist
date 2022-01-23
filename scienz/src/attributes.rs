use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use abi_stable::StableAbi;
#[cfg(feature = "sql")]
use aorist_attributes::TSQLAttribute;
use aorist_attributes::{
    AttributeValue, TAttribute, TBigQueryAttribute, TOrcAttribute, TPostgresAttribute,
    TPrestoAttribute, TSQLiteAttribute,
};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
#[cfg(feature = "sql")]
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "sql")]
use sqlparser::ast::{DataType, Expr};
#[cfg(feature = "sql")]
use std::collections::HashMap;
use std::fmt::Debug;
include!(concat!(env!("OUT_DIR"), "/attributes.rs"));

#[cfg(feature = "sql")]
struct WrappedAttribute(Attribute);
#[cfg(feature = "sql")]
pub type AttrMap = HashMap<AString, HashMap<AString, LinkedHashMap<AString, Attribute>>>;
#[cfg(feature = "sql")]
impl WrappedAttribute {
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, AString> {
        match x {
            Expr::Identifier(_) => Err(
                "Simple identifiers not supported for now. Please prefix with table name".into(),
            ),
            Expr::CompoundIdentifier(mut idents) => {
                if idents.len() != 3 {
                    return Err("Exactly 3 identifiers must be in each compound identifier.".into());
                }
                let attr_name = idents.pop().unwrap().value;
                let asset_name = idents.pop().unwrap().value;
                let dataset_name = idents.pop().unwrap().value;
                match attr.get(&dataset_name) {
                    Some(assets) => match assets.get(&asset_name) {
                        Some(ref map) => match map.get(&attr_name) {
                            Some(attr) => Ok(Self(attr.clone())),
                            None => Err(format!(
                                "Could not find attribute {} in asset {} on {} ",
                                &attr_name, &asset_name, &dataset_name,
                            )),
                        },
                        None => Err(format!(
                            "Could not find asset named {} in dataset {} ",
                            asset_name, dataset_name
                        )),
                    },
                    None => Err(format!("Could not find dataset named {} ", dataset_name)),
                }
            }
            _ => Err("Only identifiers supported as nodes".into()),
        }
    }
}
#[repr(C)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
pub enum AttributeOrValue {
    Attribute(Attribute),
    Value(AttributeValue),
}
#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for AttributeOrValue {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        if let Ok(x) = PyAttribute::extract(obj) {
            return Ok(Self::Attribute(x.inner.0.read().clone()));
        } else if let Ok(x) = AttributeValue::extract(obj) {
            return Ok(Self::Value(x));
        }
        Err(PyValueError::new_err("could not convert enum arm."))
    }
}
impl AttributeOrValue {
    pub fn as_sql(&self) -> AString {
        match &self {
            AttributeOrValue::Attribute(x) => x.get_name().clone(),
            AttributeOrValue::Value(x) => x.as_sql(),
        }
    }
    #[cfg(feature = "sql")]
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, AString> {
        match x {
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } => {
                Ok(Self::Attribute(WrappedAttribute::try_from(x, attr)?.0))
            }
            Expr::Value { .. } => Ok(Self::Value(AttributeValue::try_from(x)?)),
            _ => Err("Only identifiers or values supported as nodes".into()),
        }
    }
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
pub struct IdentityTransform {
    attribute: AttributeOrTransform,
    name: AString,
}
impl IdentityTransform {
    pub fn get_name(&self) -> AString {
        self.name.clone()
    }
    pub fn is_nullable(&self) -> bool {
        self.attribute.is_nullable()
    }
    pub fn get_comment(&self) -> AOption<AString> {
        self.attribute.get_comment()
    }
    #[cfg(feature = "sql")]
    fn get_sql_type(&self) -> DataType {
        self.attribute.get_sql_type()
    }
    pub fn get_presto_type(&self) -> AString {
        self.attribute.get_presto_type()
    }
    pub fn get_orc_type(&self) -> AString {
        self.attribute.get_orc_type()
    }
    pub fn get_sqlite_type(&self) -> AString {
        self.attribute.get_sqlite_type()
    }
    pub fn get_postgres_type(&self) -> AString {
        self.attribute.get_postgres_type()
    }
    pub fn get_bigquery_type(&self) -> AString {
        self.attribute.get_bigquery_type()
    }
}

#[repr(C)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
pub enum Transform {
    IdentityTransform(IdentityTransform),
}
#[cfg(feature = "python")]
impl IntoPy<PyObject> for Transform {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::IdentityTransform(x) => x.into_py(py),
        }
    }
}
#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for Transform {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let res = IdentityTransform::extract(ob);
        match res {
            Ok(x) => Ok(Self::IdentityTransform(x)),
            Err(x) => Err(x),
        }
    }
}
impl Transform {
    pub fn get_name(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_name(),
        }
    }
    pub fn get_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(_) => "IdentityTransform".into(),
        }
    }
    pub fn is_nullable(&self) -> bool {
        match &self {
            Transform::IdentityTransform(x) => x.is_nullable(),
        }
    }
    pub fn get_comment(&self) -> AOption<AString> {
        match &self {
            Transform::IdentityTransform(x) => x.get_comment(),
        }
    }
    #[cfg(feature = "sql")]
    fn get_sql_type(&self) -> DataType {
        match &self {
            Transform::IdentityTransform(x) => x.get_sql_type(),
        }
    }
    pub fn get_presto_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_presto_type(),
        }
    }
    pub fn get_sqlite_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_sqlite_type(),
        }
    }
    pub fn get_postgres_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_postgres_type(),
        }
    }
    pub fn get_bigquery_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_bigquery_type(),
        }
    }
    pub fn get_orc_type(&self) -> AString {
        match &self {
            Transform::IdentityTransform(x) => x.get_orc_type(),
        }
    }
}

#[repr(C)]
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, StableAbi)]
pub enum AttributeOrTransform {
    Attribute(AoristRef<AttributeEnum>),
    Transform(AoristRef<Transform>),
}

#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for AttributeOrTransform {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        if let Ok(x) = AttributeEnum::extract(obj) {
            return Ok(Self::Attribute(AoristRef(RArc::new(RRwLock::new(x)))));
        } else if let Ok(x) = Transform::extract(obj) {
            return Ok(Self::Transform(AoristRef(RArc::new(RRwLock::new(x)))));
        }
        Err(PyValueError::new_err("could not convert enum arm."))
    }
}
#[cfg(feature = "python")]
impl IntoPy<PyObject> for AttributeOrTransform {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Attribute(x) => x.0.read().clone().into_py(py),
            Self::Transform(x) => x.0.read().clone().into_py(py),
        }
    }
}
#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for Box<Transform> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let inner = Transform::extract(ob)?;
        Ok(Box::new(inner))
    }
}
impl AttributeOrTransform {
    #[cfg(feature = "python")]
    pub fn get_py_type(&self) -> PyResult<pyo3::prelude::PyObject> {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_py_type(),
            AttributeOrTransform::Transform(_) => {
                Err(PyValueError::new_err("called py_type on a transform"))
            }
        }
    }
    pub fn get_name(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_name(),
            AttributeOrTransform::Transform(x) => x.0.read().get_name(),
        }
    }
    pub fn get_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_type(),
        }
    }
    pub fn is_nullable(&self) -> bool {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().is_nullable(),
            AttributeOrTransform::Transform(x) => x.0.read().is_nullable(),
        }
    }
    pub fn is_key_type(&self) -> bool {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().is_key_type(),
            AttributeOrTransform::Transform(_) => false,
        }
    }
    pub fn as_predicted_objective(&self) -> Result<Self, AString> {
        match &self {
            AttributeOrTransform::Attribute(x) => Ok(AttributeOrTransform::Attribute(AoristRef(
                RArc::new(RRwLock::new(x.0.read().as_predicted_objective())),
            ))),
            AttributeOrTransform::Transform(_) => {
                Err("Transforms cannot be predicted objectives".into())
            }
        }
    }
    pub fn get_comment(&self) -> AOption<AString> {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_comment(),
            AttributeOrTransform::Transform(x) => x.0.read().get_comment(),
        }
    }
    #[cfg(feature = "sql")]
    pub fn get_sql_type(&self) -> DataType {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_sql_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_sql_type(),
        }
    }
    pub fn get_presto_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_presto_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_presto_type(),
        }
    }
    pub fn get_orc_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_orc_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_orc_type(),
        }
    }
    pub fn get_sqlite_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_sqlite_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_sqlite_type(),
        }
    }
    pub fn get_postgres_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_postgres_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_postgres_type(),
        }
    }
    pub fn psycopg2_value_json_serializable(&self) -> bool {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().psycopg2_value_json_serializable(),
            _ => panic!("Should only be called for Attributes"),
        }
    }
    pub fn get_bigquery_type(&self) -> AString {
        match &self {
            AttributeOrTransform::Attribute(x) => x.0.read().get_bigquery_type(),
            AttributeOrTransform::Transform(x) => x.0.read().get_bigquery_type(),
        }
    }
    pub fn is_temporal_dimension(&self) -> bool {
        match self {
            AttributeOrTransform::Attribute(x) => match *x.0.read() {
                AttributeEnum::FromPostgresTimestampWithTimeZone(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
}
