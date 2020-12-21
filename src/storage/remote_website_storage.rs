#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::encoding::Encoding;
use crate::endpoints::EndpointConfig;
use crate::layout::FileBasedStorageLayout;
use crate::location::RemoteLocation;
use crate::prefect::{
    TObjectWithPrefectCodeGen, TPrefectEncoding, TPrefectLocation, TPrefectStorage,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct RemoteStorage {
    #[constrainable]
    location: RemoteLocation,
    #[constrainable]
    layout: FileBasedStorageLayout,
    #[constrainable]
    encoding: Encoding,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl TObjectWithPythonCodeGen for RemoteStorage {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_python_imports(preamble);
        self.encoding.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for RemoteStorage {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.location.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectStorage for RemoteStorage {
    fn get_prefect_dag(&self, _schema: &DataSchema) -> Result<String, String> {
        Ok(format!(
            "{}\n{}",
            self.location.get_prefect_download_task(
                "download_remote".to_string(),
                "/tmp/materialized_file".to_string(),
            ),
            self.encoding.get_prefect_decode_tasks(
                "/tmp/materialized_file".to_string(),
                "decode_file".to_string(),
                "download_remote".to_string(),
            )
        ))
    }
    fn get_prefect_ingest_dag(
        &self,
        _path: String,
        _filename: String,
        _schema: &DataSchema,
        _templates: &HashMap<String, DatumTemplate>,
        _task_name: String,
        _upstream_task_name: String,
        _endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        Err("Ingest dag not implemented".to_string())
    }
}
