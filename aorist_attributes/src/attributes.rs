#![allow(dead_code)]
//pub use crate::sql_parser::AttrMap;
use num::Float;
use sqlparser::ast::{ColumnDef, DataType, Expr, Ident, Value};
use indoc::formatdoc;
pub trait TValue {}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
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
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
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
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
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

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum AttributeValue {
    IntegerValue(IntegerValue),
    StringValue(StringValue),
    FloatValue(FloatValue),
}
impl AttributeValue {
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

include!(concat!(env!("OUT_DIR"), "/attributes.rs"));