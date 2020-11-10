#![allow(non_snake_case)]

use aorist_derive::{PrestoBigint, PrestoReal, PrestoVarchar,
                    OrcBigint, OrcFloat, OrcString};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

macro_rules! define_attribute {
    ($element:ident, $presto_type:ident, $orc_type:ident) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone, $presto_type, $orc_type)]
        pub struct $element {
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
    };
}
macro_rules! register_attribute {
    ( $name:ident, $($element: ident),+ ) => {
        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        #[serde(tag = "type")]
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl TAttribute for $name {
            fn get_name(&self) -> &String {
                match self {
                    $(
                        $name::$element(x) => x.get_name(),
                    )+
                }
            }
            fn get_comment(&self) -> &Option<String> {
                match self {
                    $(
                        $name::$element(x) => x.get_comment(),
                    )+
                }
            }
        }
        impl TPrestoAttribute for Attribute {
            fn get_presto_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_presto_type(),
                    )+
                }
            }
        }
        impl TOrcAttribute for Attribute {
            fn get_orc_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_orc_type(),
                    )+
                }
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

define_attribute!(KeyStringIdentifier, PrestoVarchar, OrcString);
define_attribute!(NullableStringIdentifier, PrestoVarchar, OrcString);
define_attribute!(NullablePOSIXTimestamp, PrestoBigint, OrcBigint);
define_attribute!(NullableInt64, PrestoBigint, OrcBigint);
define_attribute!(NullableString, PrestoVarchar, OrcString);
define_attribute!(FloatLatitude, PrestoReal, OrcFloat);
define_attribute!(FloatLongitude, PrestoReal, OrcFloat);
define_attribute!(URI, PrestoVarchar, OrcString);
register_attribute!(
    Attribute,
    KeyStringIdentifier,
    NullableStringIdentifier,
    NullablePOSIXTimestamp,
    NullableInt64,
    NullableString,
    FloatLatitude,
    FloatLongitude,
    URI
);
