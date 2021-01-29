extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

pub mod access_policy;
pub mod airflow_singleton;
pub mod asset;
pub mod attributes;
pub mod code_block;
pub mod compression;
pub mod concept;
pub mod constraint;
pub mod constraint_block;
pub mod constraint_state;
pub mod data_setup;
pub mod dataset;
pub mod driver;
pub mod encoding;
pub mod endpoints;
pub mod error;
pub mod etl_singleton;
pub mod etl_task;
pub mod header;
pub mod imports;
pub mod layout;
pub mod location;
pub mod object;
pub mod prefect_singleton;
pub mod python;
pub mod ranger;
pub mod role;
pub mod role_binding;
pub mod schema;
pub mod storage;
pub mod storage_setup;
pub mod template;
pub mod user;
pub mod user_group;
pub mod utils;

pub use asset::Asset;
pub use asset::StaticDataTable;
pub use compression::*;
pub use data_setup::{DataSetup, ParsedDataSetup};
pub use dataset::DataSet;
pub use encoding::Encoding;
pub use header::FileHeader;
pub use location::{GCSLocation, HiveLocation, RemoteLocation, WebLocation};
pub use schema::{DataSchema, TabularSchema};
pub use storage::{HiveTableStorage, RemoteStorage};
pub use storage_setup::{RemoteImportStorageSetup, StorageSetup};
pub use template::{DatumTemplate, IdentifierTuple, KeyedStruct};

py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "Aorist Python wrapper")?;
    Ok(())
});
