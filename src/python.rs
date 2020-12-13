use crate::assets::Asset;
use crate::compression::DataCompression;
use crate::encoding::Encoding;
use crate::endpoints::EndpointConfig;
use crate::headers::FileHeader;
use crate::location::{HiveLocation, RemoteLocation};
use enum_dispatch::enum_dispatch;
use indoc::formatdoc;
use std::collections::HashMap;

#[enum_dispatch(
    HiveLocation,
    RemoteLocation,
    Asset,
    StorageSetup,
    Storage,
    Encoding,
    DataCompression,
    FileHeader
)]
pub trait TObjectWithPythonCodeGen {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>);
}
#[enum_dispatch(HiveLocation)]
pub trait TLocationWithPythonAPIClient: TObjectWithPythonCodeGen {
    fn get_python_client(&self, client_name: &String, endpoints: &EndpointConfig) -> String;
    fn get_python_create_storage(&self, client_name: &String, endpoints: &EndpointConfig)
        -> String;

    fn get_prefect_create_storage_task(
        &self,
        task_name: &String,
        client_name: &String,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) -> String {
        self.get_python_imports(preamble);
        formatdoc!(
            "@task
             def {task_name}:
                {python_create_storage}
            ",
            task_name = task_name,
            python_create_storage = self.get_python_create_storage(client_name, endpoints)
        )
        .to_string()
    }
}
