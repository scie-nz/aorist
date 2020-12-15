#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectAsset, TPrefectStorageSetup};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct StaticDataTable {
    name: String,
    #[constrainable]
    setup: StorageSetup,
    #[constrainable]
    schema: DataSchema,
}
impl StaticDataTable {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        self.setup
            .get_presto_schemas(self.get_name(), columnSchema, endpoints)
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
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.setup.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectAsset for StaticDataTable {
    fn get_prefect_dag(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        self.setup
            .get_prefect_dag(&self.schema, templates, self.get_name(), endpoints)
    }
}
