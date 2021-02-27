use core::convert::TryFrom;
use indoc::formatdoc;
use num::Float;
use sqlparser::ast::{BinaryOperator, ColumnDef, DataType, Expr, Ident, Value};
use std::collections::HashMap;
pub trait TValue {}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct IntegerValue {
    inner: i64,
}
impl IntegerValue {
    fn try_from(x: String) -> Result<Self, String> {
        match x.parse::<i64>() {
            Ok(val) => Ok(Self { inner: val }),
            Err(_) => Err(format!("Could not parse {} as int.", &x).into()),
        }
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct StringValue {
    inner: String,
}
impl StringValue {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct FloatValue {
    sign: i8,
    mantissa: u64,
    exponent: i16,
}
impl FloatValue {
    fn try_from(x: String) -> Result<Self, String> {
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
    fn try_from(x: Expr) -> Result<Self, String> {
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
}

pub trait TAttribute
where
    Self::Value: TValue,
{
    type Value;
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
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

include!(concat!(env!("OUT_DIR"), "/attributes.rs"));
include!(concat!(env!("OUT_DIR"), "/programs.rs"));

impl Attribute {
    fn try_from(
        x: Expr,
        attr: &HashMap<String, HashMap<String, Attribute>>,
    ) -> Result<Self, String> {
        match x {
            Expr::Identifier(_) => Err(
                "Simple identifiers not supported for now. Please prefix with table name".into(),
            ),
            Expr::CompoundIdentifier(mut idents) => {
                if idents.len() != 2 {
                    return Err(
                        "Exactly 2 identifiers must be in each compound identifier.".to_string()
                    );
                }
                let attr_name = idents.pop().unwrap().value;
                let asset_name = idents.pop().unwrap().value;
                match attr.get(&asset_name) {
                    Some(ref map) => match map.get(&attr_name) {
                        Some(attr) => Ok(attr.clone()),
                        None => Err(format!(
                            "Could not find attribute {} in asset {} ",
                            &attr_name, &asset_name
                        )),
                    },
                    None => Err(format!("Could not find asset named {} ", asset_name)),
                }
            }
            _ => Err("Only identifiers supported as nodes".into()),
        }
    }
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum AttributeOrValue {
    Attribute(Attribute),
    Value(AttributeValue),
}
impl AttributeOrValue {
    fn try_from(
        x: Expr,
        attr: &HashMap<String, HashMap<String, Attribute>>,
    ) -> Result<Self, String> {
        match x {
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } => {
                Ok(Self::Attribute(Attribute::try_from(x, attr)?))
            }
            Expr::Value { .. } => Ok(Self::Value(AttributeValue::try_from(x)?)),
            _ => Err("Only identifiers or values supported as nodes".into()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum PredicateInnerOrTerminal {
    PredicateTerminal(AttributeOrValue),
    PredicateInner(Box<PredicateInner>),
}
impl PredicateInnerOrTerminal {
    fn try_from(
        x: Expr,
        attr: &HashMap<String, HashMap<String, Attribute>>,
    ) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self::PredicateInner(Box::new(PredicateInner::try_from(
                x, attr,
            )?))),
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } | Expr::Value { .. } => Ok(
                Self::PredicateTerminal(AttributeOrValue::try_from(x, attr)?),
            ),
            _ => Err("Only Binary operators, identifiers or values supported as nodes".into()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct PredicateInner {
    left: PredicateInnerOrTerminal,
    right: PredicateInnerOrTerminal,
}
impl<'a> FromPyObject<'a> for Box<PredicateInner> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let inner = PredicateInner::extract(ob)?;
        Ok(Box::new(inner))
    }
}
impl PredicateInner {
    fn try_from(
        x: Expr,
        attr: &HashMap<String, HashMap<String, Attribute>>,
    ) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { left, op, right } => match op {
                BinaryOperator::Gt => Ok(Self {
                    left: PredicateInnerOrTerminal::try_from(*left, attr)?,
                    right: PredicateInnerOrTerminal::try_from(*right, attr)?,
                }),
                _ => Err("Only > operators supported.".into()),
            },
            _ => Err("Only binary operators supported.".into()),
        }
    }
}

#[aorist_concept]
pub struct Predicate {
    root: PredicateInner,
}

impl Predicate {
    fn try_from(
        x: Expr,
        attr: &HashMap<String, HashMap<String, Attribute>>,
    ) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self {
                root: PredicateInner::try_from(x, attr)?,
                constraints: Vec::new(),
                tag: None,
                uuid: None,
            }),
            _ => Err("Only binary operators supported.".into()),
        }
    }
}
