use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::python::TObjectWithPythonCodeGen;
use crate::locations::{GCSLocation, RemoteWebsiteLocation, HiveLocation};
use crate::assets::{Asset, StaticDataTable};
use crate::compressions::{DataCompression, GzipCompression};
use crate::encoding::Encoding;
use crate::headers::{FileHeader, UpperSnakeCaseCSVHeader};
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;

#[enum_dispatch(RemoteWebsiteLocation, HiveLocation, Storage, StorageSetup, Asset, DataCompression, Encoding, FileHeader)]
pub trait TObjectWithPrefectCodeGen: TObjectWithPythonCodeGen {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>);
}

pub trait TObjectWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(
        &self,
    ) -> Result<String, String>;
}

#[enum_dispatch(Asset)]
pub trait TAssetWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(
        &self,
        templates: &HashMap<String, DatumTemplate>,
    ) -> Result<String, String>;
}

#[enum_dispatch(StorageSetup)]
pub trait TStorageSetupWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(
        &self,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        table_name: &String,
    ) -> Result<String, String>;
}

#[enum_dispatch(Storage)]
pub trait TStorageWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(&self, schema: &DataSchema) -> Result<String, String>;
    fn get_prefect_ingest_dag(
        &self,
        path: String,
        filename: String,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        task_name: String,
        upstream_task_name: String
    ) -> Result<String, String>;
}

#[enum_dispatch(RemoteWebsiteLocation)]
pub trait TPrefectLocation: TObjectWithPrefectCodeGen {
    fn get_prefect_download_task(&self, task_name: String, file_name: String) -> String;
}

#[enum_dispatch(HiveLocation)]
pub trait TPrefectHiveLocation: TObjectWithPrefectCodeGen {
    fn get_prefect_upload_task(
        &self,
        task_name: String,
        file_name: String,
        local_path: String,
        alluxio_path: String
    ) -> String;
}

#[enum_dispatch(Encoding)]
pub trait TPrefectEncoding: TObjectWithPrefectCodeGen {
    fn get_prefect_decode_tasks(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String;
    fn get_prefect_encode_tasks(
        &self,
        input_file_name: String,
        output_file_name: String,
        task_name: String,
        upstream_task_name: String,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
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

#[enum_dispatch(FileHeader)]
pub trait TPrefectFileHeader: TObjectWithPrefectCodeGen {
    fn get_prefect_file_header_removal_tasks(
        &self,
        input_file_name: String,
        output_file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String;
}
