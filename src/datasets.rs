#![allow(non_snake_case)]
use crate::locations::Location;
use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyStringIdentifier{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableStringIdentifier{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NullablePOSIXTimestamp{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableInt64{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NullableString{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatLatitude{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatLongitude{name: String}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct URI{name: String}


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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyedStruct {
    name: String, attributes: Vec<Attribute>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    name: String,
    location: Location,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Object {
    DataSet(DataSet),
}
impl Object {
    pub fn to_yaml(&self) -> String {
        match self {
            Object::DataSet{..} => serde_yaml::to_string(self).unwrap(),
        }
    }
}

pub fn get_dataset() -> Object {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let dataset: Object = serde_yaml::from_str(&s).unwrap();
    dataset
}
