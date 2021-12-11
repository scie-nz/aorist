#![allow(dead_code)]
use abi_stable::std_types::ROption;
use aorist_primitives::AOption;
use indoc::formatdoc;
use num::Float;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "sql")]
use sqlparser::ast::{ColumnDef, DataType, Expr, Ident, Value};
pub trait TValue {}
use aorist_primitives::AString;

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub struct IntegerValue {
    inner: i64,
}
impl IntegerValue {
    pub fn try_from(x: String) -> Result<Self, String> {
        match x.parse::<i64>() {
            Ok(val) => Ok(Self { inner: val }),
            Err(_) => Err(format!("Could not parse {} as int.", &x).as_str().into()),
        }
    }
    pub fn as_sql(&self) -> AString {
        format!("{}", self.inner).as_str().into()
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
    pub fn as_sql(&self) -> AString {
        format!("\"{}\"", self.inner).as_str().into()
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FloatValue {
    sign: i8,
    mantissa: u64,
    exponent: i16,
}
/*#[cfg(feature = "python")]
impl pyo3::callback::IntoPyCallbackOutput<FloatValue> for Result<FloatValue, pyo3::PyErr> {
    #[inline]
    fn convert(self, _: Python) -> PyResult<FloatValue> {
        Ok(self.unwrap())
    }
}
#[cfg(feature = "python")]
impl pyo3::callback::IntoPyCallbackOutput<FloatValue> for FloatValue {
    #[inline]
    fn convert(self, _: Python) -> PyResult<FloatValue> {
        Ok(self)
    }
}
impl pyo3::callback::PyCallbackOutput for FloatValue {
    const ERR_VALUE: Self = Self { sign: -2, mantissa: 0, exponent: 0 };
}*/
#[cfg(feature = "python")]
impl std::convert::From<&pyo3::types::PyFloat> for FloatValue {
    fn from(x: &pyo3::types::PyFloat) -> Self {
        let (mantissa, exponent, sign) = Float::integer_decode(x.value());
        Self {
            sign,
            mantissa,
            exponent,
        }
    }
}
#[cfg(feature = "python")]
impl std::convert::From<&pyo3::types::PyAny> for FloatValue {
    fn from(x: &pyo3::types::PyAny) -> Self {
        let float: &pyo3::types::PyFloat = x.downcast().unwrap();
        FloatValue::from(float)
    }
}
impl std::convert::From<f64> for FloatValue {
    fn from(x: f64) -> Self {
        let (mantissa, exponent, sign) = Float::integer_decode(x);
        Self {
            sign,
            mantissa,
            exponent,
        }
    }
}
#[cfg(feature = "python")]
impl pyo3::conversion::FromPyObject<'_> for FloatValue {
    fn extract(ob: &PyAny) -> PyResult<FloatValue> {
        Ok(FloatValue::from(ob))
    }
}
impl pyo3::conversion::IntoPy<PyObject> for FloatValue {
    fn into_py(self, py: Python) -> PyObject {
        self.as_f64().into_py(py)
    }
}
impl FloatValue {
    pub fn from_f64(val: f64) -> Self {
        let (mantissa, exponent, sign) = Float::integer_decode(val);
        Self {
            sign,
            mantissa,
            exponent,
        }
    }
    pub fn try_from(x: String) -> Result<Self, String> {
        match x.parse::<f64>() {
            Ok(val) => Ok(Self::from_f64(val)),
            Err(_) => Err(format!("Could not parse {} as float.", &x).as_str().into()),
        }
    }
    pub fn as_f64(&self) -> f64 {
        let num = 2.0f64;
        let sign_f = self.sign as f64;
        let mantissa_f = self.mantissa as f64;
        let exponent_f = num.powf(self.exponent as f64);
        sign_f * mantissa_f * exponent_f
    }
    pub fn as_sql(&self) -> AString {
        let num = 2.0f64;
        let sign_f = self.sign as f64;
        let mantissa_f = self.mantissa as f64;
        let exponent_f = num.powf(self.exponent as f64);
        format!("{}", (sign_f * mantissa_f * exponent_f))
            .as_str()
            .into()
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
    pub fn as_sql(&self) -> AString {
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
    fn get_name(&self) -> AString;
    fn get_comment(&self) -> AOption<AString>;
    fn is_nullable(&self) -> bool;
    fn is_key_type() -> bool;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> AString;
    fn get_presto_schema(&self, max_attribute_length: usize) -> AString {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!("{}{}{}", self.get_name(), spaces, self.get_presto_type(),);
        if let AOption(ROption::RSome(comment)) = self.get_comment() {
            let formatted_with_comment = formatdoc!(
                "
                {first_line}
                     COMMENT '{comment}'",
                first_line = first_line,
                comment = comment.as_str().to_string().trim().replace("'", "\\'")
            );
            return formatted_with_comment.as_str().into();
        }
        first_line.as_str().into()
    }
}
pub trait TOrcAttribute: TAttribute {
    fn get_orc_type(&self) -> AString;
    fn get_orc_schema(&self) -> AString {
        format!("{}:{}", self.get_name(), self.get_orc_type())
            .as_str()
            .into()
    }
}
#[cfg(feature = "sql")]
pub trait TSQLAttribute: TAttribute {
    fn get_sql_type(&self) -> DataType;
    fn get_coldef(&self) -> ColumnDef {
        ColumnDef {
            name: Ident::new(self.get_name().as_str()),
            data_type: self.get_sql_type(),
            collation: None,
            // TODO: add comments here
            options: Vec::new(),
        }
    }
}

pub trait TSQLiteAttribute: TAttribute {
    fn get_sqlite_type(&self) -> AString;
    fn get_sqlite_coldef(&self) -> AString {
        format!("{} {}", self.get_name(), self.get_sqlite_type())
            .as_str()
            .into()
    }
}
pub trait TPostgresAttribute: TAttribute {
    fn get_postgres_type(&self) -> AString;
    fn get_postgres_coldef(&self) -> AString {
        format!(
            "{} {}{}",
            self.get_name(),
            self.get_postgres_type(),
            match self.is_nullable() {
                true => "NOT NULL",
                false => "",
            },
        )
        .as_str()
        .into()
    }
    fn psycopg2_value_json_serializable(&self) -> bool {
        true
    }
}
pub trait TBigQueryAttribute: TAttribute {
    fn get_bigquery_type(&self) -> AString;
    fn get_bigquery_coldef(&self) -> AString {
        format!("{} {}", self.get_name(), self.get_bigquery_type())
            .as_str()
            .into()
    }
}
include!(concat!(env!("OUT_DIR"), "/attributes.rs"));
