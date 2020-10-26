use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
/* This trait will implement all the abstract operations
 * we might want to undertake on a regular dataset.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GCSLocation {
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Location {
    GCSLocation { uri: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApproveAccessSelector {
    pub matchLabels: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector { matchLabels: HashMap<String, Vec<String>> },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    pub name: String,
    pub location: GCSLocation,
    pub accessPolicies: Vec<AccessPolicy>,
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Object {
    DataSet { name: String, location: Location, accessPolicies: Vec<AccessPolicy> },
}
impl Object {
    pub fn to_yaml(&self) -> String {
        match self {
            Object::DataSet{..} => serde_yaml::to_string(self).unwrap(),
            _ => "Error converting to yaml.".to_string(),
        }
    }
}

pub fn get_dataset() -> Object {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let dataset: Object = serde_yaml::from_str(&s).unwrap();
    dataset
}
