#![allow(non_snake_case)]
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use crate::templates::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticDataTable {
    name: String,
    setup: StorageSetup,
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
    ) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        self.setup.get_presto_schemas(
            self.get_name(),
            columnSchema
        )
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
    ) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schemas(templates),
        }
    }
}
