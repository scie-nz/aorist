extern crate cpython;

use cpython::{py_fn, py_module_initializer, PyResult, Python};

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

pub use airflow_singleton::AirflowSingleton;
pub use asset::Asset;
pub use asset::StaticDataTable;
pub use compression::*;
pub use concept::AoristConcept;
pub use data_setup::{DataSetup, ParsedDataSetup};
pub use dataset::DataSet;
pub use driver::Driver;
pub use encoding::Encoding;
pub use header::FileHeader;
pub use location::{GCSLocation, HiveLocation, RemoteLocation, WebLocation};
pub use schema::{DataSchema, TabularSchema};
pub use storage::{HiveTableStorage, RemoteStorage};
pub use storage_setup::{RemoteImportStorageSetup, StorageSetup};
pub use template::{DatumTemplate, IdentifierTuple, KeyedStruct};
pub use utils::get_data_setup;

fn build_from_yaml(_py: Python, filename: &str) -> PyResult<String> {
    let mut setup = get_data_setup(filename);
    setup.compute_uuids();
    setup.compute_constraints();
    setup.traverse_constrainable_children(Vec::new());
    let mut driver: Driver<AirflowSingleton> = Driver::new(&setup);
    Ok(driver.run())
}

py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "Aorist Python wrapper")?;
    m.add(
        py,
        "build_from_yaml",
        py_fn!(py, build_from_yaml(filename: &str)),
    )?;
    Ok(())
});
