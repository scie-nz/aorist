extern crate pyo3;
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
pub use attributes::attribute;
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
pub use template::{DatumTemplate, IdentifierTuple, KeyedStruct, TDatumTemplate};
pub use utils::get_data_setup;

use crate::access_policy::ApproveAccessSelector;
use crate::encoding::{CSVEncoding, TSVEncoding};
use crate::endpoints::{AlluxioConfig, EndpointConfig, GiteaConfig, PrestoConfig, RangerConfig};
use crate::header::UpperSnakeCaseCSVHeader;
use crate::layout::{
    DailyGranularity, DynamicHiveTableLayout, SingleFileLayout, StaticHiveTableLayout,
};
use crate::location::AlluxioLocation;
use crate::role::GlobalPermissionsAdmin;
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use crate::encoding::orc_encoding::ORCEncoding;

use pyo3::prelude::*;

#[pymodule]
fn aorist(py: Python, m: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "attributes")?;
    attribute(submod)?;
    m.add_submodule(submod)?;

    #[pyfn(m, "build_from_yaml")]
    fn build_from_yaml(_py: Python, filename: &str) -> PyResult<String> {
        let mut setup = get_data_setup(filename);
        setup.compute_uuids();
        setup.compute_constraints();
        setup.traverse_constrainable_children(Vec::new());
        let mut driver: Driver<AirflowSingleton> = Driver::new(&setup);
        Ok(driver.run())
    }

    m.add_class::<ApproveAccessSelector>()?;
    m.add_class::<CSVEncoding>()?;
    m.add_class::<ORCEncoding>()?;
    m.add_class::<TSVEncoding>()?;
    m.add_class::<DataSet>()?;
    m.add_class::<ParsedDataSetup>()?;
    m.add_class::<StaticDataTable>()?;
    m.add_class::<User>()?;
    m.add_class::<UpperSnakeCaseCSVHeader>()?;
    m.add_class::<SingleFileLayout>()?;
    m.add_class::<StaticHiveTableLayout>()?;
    m.add_class::<DailyGranularity>()?;
    m.add_class::<DynamicHiveTableLayout>()?;
    m.add_class::<AlluxioLocation>()?;
    m.add_class::<GCSLocation>()?;
    m.add_class::<WebLocation>()?;
    m.add_class::<GlobalPermissionsAdmin>()?;
    m.add_class::<RoleBinding>()?;
    m.add_class::<TabularSchema>()?;
    m.add_class::<HiveTableStorage>()?;
    m.add_class::<RemoteStorage>()?;
    m.add_class::<RemoteImportStorageSetup>()?;
    m.add_class::<IdentifierTuple>()?;
    m.add_class::<KeyedStruct>()?;
    m.add_class::<UserGroup>()?;
    m.add_class::<EndpointConfig>()?;
    m.add_class::<AlluxioConfig>()?;
    m.add_class::<GiteaConfig>()?;
    m.add_class::<RangerConfig>()?;
    m.add_class::<PrestoConfig>()?;
    m.add_class::<GzipCompression>()?;
    Ok(())
}
