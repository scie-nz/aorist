#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use crate::storage::Storage;
use crate::encoding::Encoding;
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticDataTable {
    storage: Storage,
    encoding: Encoding,
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        columnSchema
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schema(templates),
        }
    }
}
