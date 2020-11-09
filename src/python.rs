use indoc::formatdoc;
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::locations::{RemoteWebsiteLocation, HiveLocation};
use crate::assets::Asset;
use crate::encoding::Encoding;
use crate::compressions::{DataCompression};

#[enum_dispatch(HiveLocation, RemoteWebsiteLocation, Asset, StorageSetup, Storage, Encoding, DataCompression)]
pub trait TObjectWithPythonCodeGen {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>);
}
pub trait TLocationWithPythonAPIClient: TObjectWithPythonCodeGen {
    fn get_python_client(&self, client_name: &String) -> String;
    fn get_python_create_storage(&self, client_name: &String) -> String;

    fn get_prefect_create_storage_task(
        &self,
        task_name: &String,
        client_name: &String,
        preamble: &mut HashMap<String, String>,
    ) -> String {
        self.get_python_imports(preamble);
        formatdoc!(
            "@task
             def {task_name}:
                {python_create_storage}
            ",
            task_name = task_name,
            python_create_storage = self.get_python_create_storage(client_name)
        )
        .to_string()
    }
}
