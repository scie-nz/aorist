use core::convert::TryFrom;
use indoc::formatdoc;
use sqlparser::ast::{BinaryOperator, ColumnDef, DataType, Expr, Ident};
pub trait TValue {}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct IntegerValue {
    inner: i64,
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct StringValue {
    inner: String,
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub struct FloatValue {
    sign: i8,
    mantissa: usize,
    exponent: usize,
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

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum AttributeOrValue {
    Attribute(Attribute),
    AttributeValue(AttributeValue),
}
impl TryFrom<Expr> for AttributeOrValue {
    type Error = String;
    fn try_from(x: Expr) -> Result<Self, String> {
        Err("bla".into())
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, FromPyObject)]
pub enum PredicateInnerOrTerminal {
    PredicateTerminal(AttributeOrValue),
    PredicateInner(Box<PredicateInner>),
}
impl TryFrom<Expr> for PredicateInnerOrTerminal {
    type Error = String;
    fn try_from(x: Expr) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => {
                Ok(Self::PredicateInner(Box::new(PredicateInner::try_from(x)?)))
            }
            Expr::Identifier { .. } | Expr::CompoundIdentifier { .. } | Expr::Value { .. } => {
                Ok(Self::PredicateTerminal(AttributeOrValue::try_from(x)?))
            }
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
impl TryFrom<Expr> for PredicateInner {
    type Error = String;
    fn try_from(x: Expr) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { left, op, right } => match op {
                BinaryOperator::Gt => Ok(Self {
                    left: PredicateInnerOrTerminal::try_from(*left)?,
                    right: PredicateInnerOrTerminal::try_from(*right)?,
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

impl TryFrom<Expr> for Predicate {
    type Error = String;
    fn try_from(x: Expr) -> Result<Self, String> {
        match x {
            Expr::BinaryOp { .. } => Ok(Self {
                root: PredicateInner::try_from(x)?,
                constraints: Vec::new(),
                tag: None,
                uuid: None,
            }),
            _ => Err("Only binary operators supported.".into()),
        }
    }
}
