#![allow(non_snake_case)]
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectCompression};
use crate::python::TObjectWithPythonCodeGen;
use aorist_derive::BlankPrefectPreamble;
use enum_dispatch::enum_dispatch;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, BlankPrefectPreamble)]
pub struct GzipCompression {}
impl TObjectWithPythonCodeGen for GzipCompression {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        preamble.insert(
            "import_shell_task".to_string(),
            indoc! {"from prefect.tasks.shell import ShellTask"}
            .to_string(),
        );
    }
}
impl TPrefectCompression for GzipCompression {
    fn get_prefect_decompress_task(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command='gunzip {file_name}.gz',
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name = task_name,
            upstream_task_name = upstream_task_name,
            file_name = file_name
        )
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DataCompression {
    GzipCompression(GzipCompression),
}
