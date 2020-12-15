#![allow(non_snake_case)]

use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::encoding::Encoding;
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::layout::HiveStorageLayout;
use crate::location::HiveLocation;
use crate::prefect::{
    TObjectWithPrefectCodeGen, TPrefectEncoding, TPrefectHiveLocation, TPrefectStorage,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct HiveTableStorage {
    #[constrainable]
    location: HiveLocation,
    #[constrainable]
    layout: HiveStorageLayout,
    #[constrainable]
    encoding: Encoding,
}
impl THiveTableCreationTagMutator for HiveTableStorage {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        self.encoding
            .populate_table_creation_tags(tags, endpoints)?;
        self.location.populate_table_creation_tags(tags, endpoints)
    }
}
impl TObjectWithPythonCodeGen for HiveTableStorage {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for HiveTableStorage {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.location.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectStorage for HiveTableStorage {
    fn get_prefect_dag(&self, _schema: &DataSchema) -> Result<String, String> {
        Err("Ingest dag not implemented".to_string())
    }
    fn get_prefect_ingest_dag(
        &self,
        local_path: String,
        filename: String,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        task_name: String,
        upstream_task_name: String,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        Ok(format!(
            "{}\n{}",
            self.encoding.get_prefect_encode_tasks(
                format!("{}/{}", &local_path, &filename).to_string(),
                format!("{}.encoded", filename),
                format!("{}_encode", task_name).to_string(),
                upstream_task_name,
                schema,
                templates,
            ),
            self.location.get_prefect_upload_task(
                format!("{}.encoded", filename),
                local_path,
                format!("{}_upload", task_name).to_string(),
                format!("{}_encode", task_name).to_string(),
                endpoints,
            ),
        ))
    }
}
