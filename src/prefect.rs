use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::python::TObjectWithPythonCodeGen;
use crate::locations::{GCSLocation, RemoteWebsiteLocation, HiveLocation};
use crate::assets::{Asset, StaticDataTable};
use crate::compressions::{DataCompression, GzipCompression};
use crate::encoding::Encoding;

#[enum_dispatch(RemoteWebsiteLocation, HiveLocation, Storage, StorageSetup, Asset, DataCompression, Encoding)]
pub trait TObjectWithPrefectCodeGen: TObjectWithPythonCodeGen {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>);
}

#[enum_dispatch(Asset, StorageSetup, Storage)]
pub trait TObjectWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(&self) -> Result<String, String>;
}

#[enum_dispatch(RemoteWebsiteLocation)]
pub trait TPrefectLocation: TObjectWithPrefectCodeGen {
    fn get_prefect_download_task(&self, task_name: String, file_name: String) -> String;
}

#[enum_dispatch(Encoding)]
pub trait TPrefectEncoding: TObjectWithPrefectCodeGen {
    fn get_prefect_decode_tasks(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String;
}

#[enum_dispatch(DataCompression)]
pub trait TPrefectCompression: TObjectWithPrefectCodeGen {
    fn get_prefect_decompress_task(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String;
}
