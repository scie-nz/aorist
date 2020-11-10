#![allow(non_snake_case)]

use crate::compressions::DataCompression;
use crate::headers::FileHeader;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{
    TObjectWithPrefectCodeGen, TPrefectCompression, TPrefectEncoding, TPrefectFileHeader,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;
use aorist_derive::{BlankPrefectPreamble, NoPythonImports};
use enum_dispatch::enum_dispatch;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CSVEncoding {
    compression: DataCompression,
    header: FileHeader,
}
impl THiveTableCreationTagMutator for CSVEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
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
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.compression.get_prefect_preamble(preamble);
        self.header.get_prefect_preamble(preamble)
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, NoPythonImports, BlankPrefectPreamble)]
pub struct ORCEncoding {}
impl THiveTableCreationTagMutator for ORCEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        tags.insert("format".to_string(), "ORC".to_string());
        Ok(())
    }
}
impl TPrefectEncoding for ORCEncoding {
    fn get_prefect_decode_tasks(
        &self,
        _file_name: String,
        _task_name: String,
        _upstream_task_name: String,
    ) -> String {
        "".to_string()
    }
    fn get_prefect_encode_tasks(
        &self,
        input_file_name: String,
        output_file_name: String,
        task_name: String,
        upstream_task_name: String,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
    ) -> String {
        let orc_schema = schema.get_orc_schema(templates);
        let command = format!(
            "csv-import {} {} {}",
            orc_schema, input_file_name, output_file_name,
        );
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command='{command}',
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name = task_name,
            upstream_task_name = upstream_task_name,
            command = command,
        )
        .to_string()
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
