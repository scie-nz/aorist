#![allow(non_snake_case)]

use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectFileHeader};
use crate::python::TObjectWithPythonCodeGen;
use aorist_derive::BlankPrefectPreamble;
use enum_dispatch::enum_dispatch;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::data_setup::EndpointConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, BlankPrefectPreamble)]
pub struct UpperSnakeCaseCSVHeader {}

impl TObjectWithPythonCodeGen for UpperSnakeCaseCSVHeader {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        preamble.insert(
            "import_shell_task".to_string(),
            indoc! {"
                from prefect.tasks.shell import ShellTask
            "}
            .to_string(),
        );
    }
}
impl TPrefectFileHeader for UpperSnakeCaseCSVHeader {
    fn get_prefect_file_header_removal_tasks(
        &self,
        input_file_name: String,
        output_file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command='tail -n +2 {input_file_name} > {output_file_name} ',
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name = task_name,
            upstream_task_name = upstream_task_name,
            input_file_name = input_file_name,
            output_file_name = output_file_name,
        )
    }
}
#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileHeader {
    UpperSnakeCaseCSVHeader(UpperSnakeCaseCSVHeader),
}
