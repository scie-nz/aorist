#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectFileHeader};
use crate::python::TObjectWithPythonCodeGen;
use aorist_concept::Constrainable;
use aorist_derive::BlankPrefectPreamble;
use derivative::Derivative;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, BlankPrefectPreamble, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct UpperSnakeCaseCSVHeader {
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}

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
