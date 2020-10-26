#![allow(non_snake_case)]

use crate::locations::RemoteWebsiteLocation;
use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::templates::DatumTemplate;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GzipCompression {}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataCompression {
    GzipCompression(GzipCompression),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpperSnakeCaseCSVHeader {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StorageLayout {
    SingleFileLayout(SingleFileLayout),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CSVEncoding {
    compression: DataCompression,
    header: FileHeader,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteWebsiteStorage {
    location: RemoteWebsiteLocation,
    layout: StorageLayout,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Storage {
    RemoteWebsiteStorage(RemoteWebsiteStorage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TabularSchema {
    datumTemplateName: String,
    attributes: Vec<String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum DataSchema {
    TabularSchema(TabularSchema),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticDataTable {
    storage: Storage,
    encoding: Encoding,
    schema: DataSchema,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum Table {
    StaticDataTable(StaticDataTable),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    name: String,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
    tables: Vec<Table>,
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
