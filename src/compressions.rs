#![allow(non_snake_case)]
use indoc::indoc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GzipCompression {}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DataCompression {
    GzipCompression(GzipCompression),
}
impl GzipCompression {
    pub fn get_prefect_preamble(&self) -> HashMap<String, String> {
        let mut preamble = HashMap::new();
        preamble.insert(
            "import_shell_task".to_string(),
            indoc! {"
                from prefect.tasks.shell import ShellTask
            "}.to_string()
        );
        preamble
    }
    pub fn get_prefect_download_task(&self, file_name: String,
                                     task_name: String,
                                     upstream_task_name: String) -> String {
        format!(
            indoc! {
                "
                    {task_name} = ShellTask(
                        command='gunzip {file_name}',
                    )(upstream_tasks=[{upstream_task_name}])
                "
            },
            task_name=task_name,
            upstream_task_name=upstream_task_name,
            file_name=file_name
        )
    }
}
