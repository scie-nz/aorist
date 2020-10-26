#![allow(non_snake_case)]

use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::templates::DatumTemplate;
use crate::assets::Asset;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    name: String,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
    assets: Vec<Asset>,
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
    pub fn get_presto_schemas(&self) -> String {
        match self {
            Object::DataSet(x) => x.datumTemplates[0].get_presto_schema(),
        }
    }
}

pub fn get_dataset() -> Object {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let dataset: Object = serde_yaml::from_str(&s).unwrap();
    dataset
}
