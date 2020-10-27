#![allow(non_snake_case)]

use aorist_derive::{PrestoBigint, PrestoReal, PrestoVarchar};
use serde::{Serialize, Deserialize};
use indoc::formatdoc;

macro_rules! attribute {
    ($element:ident, $presto_type:ident) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, $presto_type)]
        pub struct $element{
            name: String,
            comment: Option<String>,
        }
        impl TAttribute for $element {
            fn get_name(&self) -> &String {
                &self.name
            }
            fn get_comment(&self) -> &Option<String> {
                &self.comment
            }
        }
    }
}

pub trait TAttribute {
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!(
            "{}{}{}",
            self.get_name(),
            spaces,
            self.get_presto_type(),
        );
        if let Some(comment) = self.get_comment() {
            let formatted_with_comment = formatdoc!("
                {first_line}
                     COMMENT '{comment}'",
                first_line=first_line,
                comment=comment.trim().replace("'","\\'").to_string()
            );
            return formatted_with_comment
        }
        first_line
    }
}

attribute!(KeyStringIdentifier, PrestoVarchar);
attribute!(NullableStringIdentifier, PrestoVarchar);
attribute!(NullablePOSIXTimestamp, PrestoBigint);
attribute!(NullableInt64, PrestoBigint);
attribute!(NullableString, PrestoVarchar);
attribute!(FloatLatitude, PrestoReal);
attribute!(FloatLongitude, PrestoReal);
attribute!(URI, PrestoVarchar);

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Attribute {
    KeyStringIdentifier(KeyStringIdentifier),
    NullableStringIdentifier(NullableStringIdentifier),
    NullablePOSIXTimestamp(NullablePOSIXTimestamp),
    NullableInt64(NullableInt64),
    NullableString(NullableString),
    FloatLatitude(FloatLatitude),
    FloatLongitude(FloatLongitude),
    URI(URI),
}
impl TAttribute for Attribute {
    fn get_name(&self) -> &String {
        match self {
            Attribute::KeyStringIdentifier(x) => x.get_name(),
            Attribute::NullableStringIdentifier(x) => x.get_name(),
            Attribute::NullablePOSIXTimestamp(x) => x.get_name(),
            Attribute::NullableInt64(x) => x.get_name(),
            Attribute::FloatLatitude(x) => x.get_name(),
            Attribute::FloatLongitude(x) => x.get_name(),
            Attribute::URI(x) => x.get_name(),
            Attribute::NullableString(x) => x.get_name(),
        }
    }
    fn get_comment(&self) -> &Option<String> {
        match self {
            Attribute::KeyStringIdentifier(x) => x.get_comment(),
            Attribute::NullableStringIdentifier(x) => x.get_comment(),
            Attribute::NullablePOSIXTimestamp(x) => x.get_comment(),
            Attribute::NullableInt64(x) => x.get_comment(),
            Attribute::FloatLatitude(x) => x.get_comment(),
            Attribute::FloatLongitude(x) => x.get_comment(),
            Attribute::URI(x) => x.get_comment(),
            Attribute::NullableString(x) => x.get_comment(),
        }
    }
}
impl TPrestoAttribute for Attribute {
    fn get_presto_type(&self) -> String {
        match self {
            Attribute::KeyStringIdentifier(x) => x.get_presto_type(),
            Attribute::NullableStringIdentifier(x) => x.get_presto_type(),
            Attribute::NullablePOSIXTimestamp(x) => x.get_presto_type(),
            Attribute::NullableInt64(x) => x.get_presto_type(),
            Attribute::FloatLatitude(x) => x.get_presto_type(),
            Attribute::FloatLongitude(x) => x.get_presto_type(),
            Attribute::URI(x) => x.get_presto_type(),
            Attribute::NullableString(x) => x.get_presto_type(),
        }
    }
}

