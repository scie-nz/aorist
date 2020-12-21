#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectEncoding};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use aorist_derive::{BlankPrefectPreamble, NoPythonImports};
use derivative::Derivative;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(
    Derivative, Serialize, Deserialize, Clone, NoPythonImports, BlankPrefectPreamble, Constrainable,
)]
#[derivative(PartialEq, Debug)]
pub struct ORCEncoding {
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl THiveTableCreationTagMutator for ORCEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        _endpoints: &EndpointConfig,
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
