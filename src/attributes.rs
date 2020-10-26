#![allow(non_snake_case)]

use aorist_derive::{PrestoBigint, PrestoReal, PrestoVarchar};
use serde::{Serialize, Deserialize};

pub trait TAttribute {
    fn get_name(&self) -> &String;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        format!("{}{}{}", self.get_name(), spaces, self.get_presto_type())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoVarchar)]
pub struct KeyStringIdentifier{name: String}
impl TAttribute for KeyStringIdentifier{
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoVarchar)]
pub struct NullableStringIdentifier{name: String}
impl TAttribute for NullableStringIdentifier {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoBigint)]
pub struct NullablePOSIXTimestamp{name: String}
impl TAttribute for NullablePOSIXTimestamp {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoBigint)]
pub struct NullableInt64{name: String}
impl TAttribute for NullableInt64 {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoVarchar)]
pub struct NullableString{name: String}
impl TAttribute for NullableString {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoReal)]
pub struct FloatLatitude{name: String}
impl TAttribute for FloatLatitude {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoReal)]
pub struct FloatLongitude{name: String}
impl TAttribute for FloatLongitude {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, PrestoVarchar)]
pub struct URI{name: String}
impl TAttribute for URI {
    fn get_name(&self) -> &String {
        &self.name
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

