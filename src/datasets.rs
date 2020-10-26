#![allow(non_snake_case)]
use crate::locations::Location;
use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Attribute {
    KeyStringIdentifier {name: String},
    NullableStringIdentifier {name: String},
    NullablePOSIXTimeStamp {name: String},
    NullableInt64 {name: String},
    NullableString {name: String},
    FloatLatitude {name: String},
    FloatLongitude {name: String},
    URI {name: String},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct{name: String, attributes: Vec<Attribute>},
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Object {
    DataSet {
        name: String,
        location: Location,
        accessPolicies: Vec<AccessPolicy>,
        datumTemplates: Vec<DatumTemplate>,
    },
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
