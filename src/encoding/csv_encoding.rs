#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::header::FileHeader;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{
    TObjectWithPrefectCodeGen, TPrefectCompression, TPrefectEncoding, TPrefectFileHeader,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct CSVEncoding {
    #[constrainable]
    compression: DataCompression,
    #[constrainable]
    header: FileHeader,
}
impl THiveTableCreationTagMutator for CSVEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        _endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        tags.insert("format".to_string(), "CSV".to_string());
        Ok(())
    }
}
impl TObjectWithPythonCodeGen for CSVEncoding {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.compression.get_python_imports(preamble);
        self.header.get_python_imports(preamble)
    }
}
impl TObjectWithPrefectCodeGen for CSVEncoding {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.compression.get_prefect_preamble(preamble, endpoints);
        self.header.get_prefect_preamble(preamble, endpoints)
    }
}
impl TPrefectEncoding for CSVEncoding {
    fn get_prefect_decode_tasks(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        format!(
            "{}\n{}",
            self.compression.get_prefect_decompress_task(
                file_name.clone(),
                format!("{}_decompress", &task_name).to_string(),
                upstream_task_name.clone(),
            ),
            self.header.get_prefect_file_header_removal_tasks(
                file_name.clone(),
                format!("{}.no_header", &file_name).to_string(),
                format!("{}_remove_header", &task_name).to_string(),
                format!("{}_decompress", &task_name).to_string(),
            )
        )
    }
    fn get_prefect_encode_tasks(
        &self,
        _input_file_name: String,
        _output_file_name: String,
        _task_name: String,
        _upstream_task_name: String,
        _schema: &DataSchema,
        _templates: &HashMap<String, DatumTemplate>,
    ) -> String {
        "".to_string()
    }
}
