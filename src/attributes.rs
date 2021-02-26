use indoc::formatdoc;
use sqlparser::ast::{ColumnDef, DataType, Ident};
pub trait TValue {}

#[aorist_concept]
pub struct IntegerValue {
    inner: i64,
}
#[aorist_concept]
pub struct StringValue {
    inner: String
}
#[aorist_concept]
pub struct FloatValue{
    sign: i8,
    mantissa: usize,
    exponent: usize,
}

impl TValue for IntegerValue {}
impl TValue for StringValue {}
impl TValue for FloatValue {}

#[aorist_concept]
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

#[aorist_concept]
pub enum AttributeOrValue {
    Attribute(Attribute),
    AttributeValue(AttributeValue),    
}
/*
#[aorist_concept]
pub struct EqualityPredicate {
    left: AttributeOrValue,
    right: AttributeOrValue,
}*/
