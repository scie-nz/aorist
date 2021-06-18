#![allow(dead_code)]
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use indoc::formatdoc;
use linked_hash_map::LinkedHashMap;
use num::Float;
use paste::paste;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "sql")]
use sqlparser::ast::{ColumnDef, DataType, Expr, Ident, Value};
use std::collections::HashMap;
use uuid::Uuid;
pub trait TValue {}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub struct IntegerValue {
    inner: i64,
}
impl IntegerValue {
    pub fn try_from(x: String) -> Result<Self, String> {
        match x.parse::<i64>() {
            Ok(val) => Ok(Self { inner: val }),
            Err(_) => Err(format!("Could not parse {} as int.", &x).into()),
        }
    }
    pub fn as_sql(&self) -> String {
        format!("{}", self.inner).to_string()
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub struct StringValue {
    inner: String,
}
impl StringValue {
    pub fn from(inner: String) -> Self {
        Self { inner }
    }
    pub fn as_sql(&self) -> String {
        format!("\"{}\"", self.inner).to_string()
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub struct FloatValue {
    sign: i8,
    mantissa: u64,
    exponent: i16,
}
impl FloatValue {
    pub fn try_from(x: String) -> Result<Self, String> {
        match x.parse::<f64>() {
            Ok(val) => {
                let (mantissa, exponent, sign) = Float::integer_decode(val);
                Ok(Self {
                    sign,
                    mantissa,
                    exponent,
                })
            }
            Err(_) => Err(format!("Could not parse {} as float.", &x).into()),
        }
    }
    pub fn as_sql(&self) -> String {
        let num = 2.0f64;
        let sign_f = self.sign as f64;
        let mantissa_f = self.mantissa as f64;
        let exponent_f = num.powf(self.exponent as f64);
        format!("{}", (sign_f * mantissa_f * exponent_f)).to_string()
    }
}

impl TValue for IntegerValue {}
impl TValue for StringValue {}
impl TValue for FloatValue {}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum AttributeValue {
    IntegerValue(IntegerValue),
    StringValue(StringValue),
    FloatValue(FloatValue),
}
impl AttributeValue {
    #[cfg(feature = "sql")]
    pub fn try_from(x: Expr) -> Result<Self, String> {
        match x {
            Expr::Value(val) => match val {
                Value::Number(inner, _) => match inner.contains(".") {
                    true => Ok(Self::FloatValue(FloatValue::try_from(inner)?)),
                    false => Ok(Self::IntegerValue(IntegerValue::try_from(inner)?)),
                },
                Value::SingleQuotedString(inner) => Ok(Self::StringValue(StringValue::from(inner))),
                _ => Err("Only numbers and singlue quoted strings supported as literals".into()),
            },
            _ => Err("Only values supported as nodes".into()),
        }
    }
    pub fn as_sql(&self) -> String {
        match &self {
            AttributeValue::IntegerValue(x) => x.as_sql(),
            AttributeValue::StringValue(x) => x.as_sql(),
            AttributeValue::FloatValue(x) => x.as_sql(),
        }
    }
}

pub trait TAttribute
where
    Self::Value: TValue,
{
    type Value;
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
    fn is_nullable(&self) -> bool;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!("{}{}{}", self.get_name(), spaces, self.get_presto_type(),);
        if let Some(comment) = self.get_comment() {
            let formatted_with_comment = formatdoc!(
                "
                {first_line}
                     COMMENT '{comment}'",
                first_line = first_line,
                comment = comment.trim().replace("'", "\\'").to_string()
            );
            return formatted_with_comment;
        }
        first_line
    }
}
pub trait TOrcAttribute: TAttribute {
    fn get_orc_type(&self) -> String;
    fn get_orc_schema(&self) -> String {
        format!("{}:{}", self.get_name(), self.get_orc_type()).to_string()
    }
}
#[cfg(feature = "sql")]
pub trait TSQLAttribute: TAttribute {
    fn get_sql_type(&self) -> DataType;
    fn get_coldef(&self) -> ColumnDef {
        ColumnDef {
            name: Ident::new(self.get_name()),
            data_type: self.get_sql_type(),
            collation: None,
            // TODO: add comments here
            options: Vec::new(),
        }
    }
}

pub trait TSQLiteAttribute: TAttribute {
    fn get_sqlite_type(&self) -> String;
    fn get_sqlite_coldef(&self) -> String {
        format!("{} {}", self.get_name(), self.get_sqlite_type()).to_string()
    }
}
pub trait TPostgresAttribute: TAttribute {
    fn get_postgres_type(&self) -> String;
    fn get_postgres_coldef(&self) -> String {
        format!(
            "{} {}{}",
            self.get_name(),
            self.get_postgres_type(),
            match self.is_nullable() {
                true => "NOT NULL",
                false => "",
            },
        )
        .to_string()
    }
    fn psycopg2_value_json_serializable(&self) -> bool {
        true
    }
}
pub trait TBigQueryAttribute: TAttribute {
    fn get_bigquery_type(&self) -> String;
    fn get_bigquery_coldef(&self) -> String {
        format!("{} {}", self.get_name(), self.get_bigquery_type()).to_string()
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum AttributeOrValue {
    Attribute(Attribute),
    Value(AttributeValue),
}
#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for AttributeOrValue {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        if let Ok(x) = Attribute::extract(obj) {
            return Ok(Self::Attribute(x));
        } else if let Ok(x) = AttributeValue::extract(obj) {
            return Ok(Self::Value(x));
        }
        Err(PyValueError::new_err("could not convert enum arm."))
    }
}
impl AttributeOrValue {
    pub fn as_sql(&self) -> String {
        match &self {
            AttributeOrValue::Attribute(x) => x.get_name().clone(),
            AttributeOrValue::Value(x) => x.as_sql(),
        }
    }
    #[cfg(feature = "sql")]
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } => {
                Ok(Self::Attribute(WrappedAttribute::try_from(x, attr)?.0))
            }
            Expr::Value { .. } => Ok(Self::Value(AttributeValue::try_from(x)?)),
            _ => Err("Only identifiers or values supported as nodes".into()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct IdentityTransform {
    attribute: AttributeOrTransform,
    name: String,
}

#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for IdentityTransform {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let attribute = obj.getattr("attribute")?;
        let name = obj.getattr("name")?;
        Ok(Self {
            attribute: AttributeOrTransform::extract(attribute)?,
            name: String::extract(name)?,
        })
    }
}
impl IdentityTransform {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn is_nullable(&self) -> bool {
        self.attribute.is_nullable()
    }
    pub fn get_comment(&self) -> &Option<String> {
        self.attribute.get_comment()
    }
    fn get_sql_type(&self) -> DataType {
        self.attribute.get_sql_type()
    }
    pub fn get_presto_type(&self) -> String {
        self.attribute.get_presto_type()
    }
    pub fn get_orc_type(&self) -> String {
        self.attribute.get_orc_type()
    }
    pub fn get_sqlite_type(&self) -> String {
        self.attribute.get_sqlite_type()
    }
    pub fn get_postgres_type(&self) -> String {
        self.attribute.get_postgres_type()
    }
    pub fn get_bigquery_type(&self) -> String {
        self.attribute.get_bigquery_type()
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum Transform {
    IdentityTransform(IdentityTransform),
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
    pub fn get_name(&self) -> &String {
        match &self {
            Transform::IdentityTransform(x) => x.get_name(),
        }
    }
    pub fn get_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(_) => "IdentityTransform".to_string(),
        }
    }
    pub fn is_nullable(&self) -> bool {
        match &self {
            Transform::IdentityTransform(x) => x.is_nullable(),
        }
    }
    pub fn get_comment(&self) -> &Option<String> {
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
    pub fn get_presto_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(x) => x.get_presto_type(),
        }
    }
    pub fn get_sqlite_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(x) => x.get_sqlite_type(),
        }
    }
    pub fn get_postgres_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(x) => x.get_postgres_type(),
        }
    }
    pub fn get_bigquery_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(x) => x.get_bigquery_type(),
        }
    }
    pub fn get_orc_type(&self) -> String {
        match &self {
            Transform::IdentityTransform(x) => x.get_orc_type(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum AttributeOrTransform {
    Attribute(AttributeEnum),
    Transform(Box<Transform>),
}

#[cfg(feature = "python")]
impl<'a> FromPyObject<'a> for AttributeOrTransform {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        if let Ok(x) = AttributeEnum::extract(obj) {
            return Ok(Self::Attribute(x));
        } else if let Ok(x) = Box::<Transform>::extract(obj) {
            return Ok(Self::Transform(x));
        }
        Err(PyValueError::new_err("could not convert enum arm."))
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
    pub fn get_name(&self) -> &String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_name(),
            AttributeOrTransform::Transform(x) => x.get_name(),
        }
    }
    pub fn get_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_type(),
            AttributeOrTransform::Transform(x) => x.get_type(),
        }
    }
    pub fn is_nullable(&self) -> bool {
        match &self {
            AttributeOrTransform::Attribute(x) => x.is_nullable(),
            AttributeOrTransform::Transform(x) => x.is_nullable(),
        }
    }
    pub fn as_predicted_objective(&self) -> Result<Self, String> {
        match &self {
            AttributeOrTransform::Attribute(x) => {
                Ok(AttributeOrTransform::Attribute(x.as_predicted_objective()))
            }
            AttributeOrTransform::Transform(_) => {
                Err("Transforms cannot be predicted objectives".to_string())
            }
        }
    }
    pub fn get_comment(&self) -> &Option<String> {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_comment(),
            AttributeOrTransform::Transform(x) => x.get_comment(),
        }
    }
    #[cfg(feature = "sql")]
    pub fn get_sql_type(&self) -> DataType {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_sql_type(),
            AttributeOrTransform::Transform(x) => x.get_sql_type(),
        }
    }
    pub fn get_presto_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_presto_type(),
            AttributeOrTransform::Transform(x) => x.get_presto_type(),
        }
    }
    pub fn get_orc_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_orc_type(),
            AttributeOrTransform::Transform(x) => (*x).get_orc_type(),
        }
    }
    pub fn get_sqlite_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_sqlite_type(),
            AttributeOrTransform::Transform(x) => (*x).get_sqlite_type(),
        }
    }
    pub fn get_postgres_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_postgres_type(),
            AttributeOrTransform::Transform(x) => (*x).get_postgres_type(),
        }
    }
    pub fn psycopg2_value_json_serializable(&self) -> bool {
        match &self {
            AttributeOrTransform::Attribute(x) => x.psycopg2_value_json_serializable(),
            _ => panic!("Should only be called for Attributes"),
        }
    }
    pub fn get_bigquery_type(&self) -> String {
        match &self {
            AttributeOrTransform::Attribute(x) => x.get_bigquery_type(),
            AttributeOrTransform::Transform(x) => (*x).get_bigquery_type(),
        }
    }
    pub fn is_temporal_dimension(&self) -> bool {
        match self {
            AttributeOrTransform::Attribute(AttributeEnum::FromPostgresTimestampWithTimeZone(
                _,
            )) => true,
            _ => false,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/attributes.rs"));

struct WrappedAttribute(Attribute);
pub type AttrMap = HashMap<String, HashMap<String, LinkedHashMap<String, Attribute>>>;

impl WrappedAttribute {
    pub fn try_from(x: Expr, attr: &AttrMap) -> Result<Self, String> {
        match x {
            Expr::Identifier(_) => Err(
                "Simple identifiers not supported for now. Please prefix with table name".into(),
            ),
            Expr::CompoundIdentifier(mut idents) => {
                if idents.len() != 3 {
                    return Err(
                        "Exactly 3 identifiers must be in each compound identifier.".to_string()
                    );
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
