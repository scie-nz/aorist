#![allow(non_snake_case)]
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use crate::templates::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::prefect::{TObjectWithPrefectCodeGen, TObjectWithPrefectDAGCodeGen};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticDataTable {
    name: String,
    setup: StorageSetup,
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schemas(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        self.setup.get_presto_schemas(self.get_name(), columnSchema)
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
impl TObjectWithPythonCodeGen for StaticDataTable {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.setup.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for StaticDataTable {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.setup.get_prefect_preamble(preamble);
    }
}
impl TObjectWithPrefectDAGCodeGen for StaticDataTable {
    fn get_prefect_dag(&self) -> Result<String, String> {
        self.setup.get_prefect_dag()
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Asset {
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schemas(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schemas(templates),
        }
    }
}
