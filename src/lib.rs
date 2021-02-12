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
pub mod layout;
pub mod location;
pub mod object;
pub mod prefect_singleton;
pub mod python;
pub mod python_singleton;
pub mod role;
pub mod role_binding;
pub mod schema;
pub mod storage;
pub mod storage_setup;
pub mod template;
pub mod user;
pub mod user_group;
pub mod utils;

pub use access_policy::*;
pub use airflow_singleton::*;
pub use asset::*;
pub use asset::*;
pub use attributes::*;
pub use compression::*;
pub use concept::*;
pub use data_setup::*;
pub use dataset::*;
pub use driver::*;
pub use encoding::*;
pub use endpoints::*;
pub use header::*;
pub use layout::*;
pub use location::*;
pub use prefect_singleton::*;
pub use python_singleton::*;
pub use role::*;
pub use role_binding::*;
pub use schema::*;
pub use storage::*;
pub use storage_setup::*;
pub use template::*;
pub use user::*;
pub use user_group::*;
pub use utils::*;

use aorist_primitives::TAttribute;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn default_tabular_schema(datum_template: InnerDatumTemplate) -> InnerTabularSchema {
    InnerTabularSchema {
        datumTemplateName: datum_template.get_name(),
        attributes: datum_template
            .get_attributes()
            .iter()
            .map(|x| x.get_name().clone())
            .collect(),
        tag: None,
    }
}

#[pyfunction]
pub fn dag(inner: InnerUniverse, constraints: Vec<String>, mode: &str) -> PyResult<String> {
    let mut universe = Universe::from(inner);
    universe.compute_uuids();
    universe.compute_constraints();
    universe.traverse_constrainable_children(Vec::new());
    python::format_code(match mode {
        "airflow" => {
            Driver::<AirflowDAG>::new(&universe, Some(constraints.into_iter().collect())).run()
        }
        "prefect" => {
            Driver::<PrefectDAG>::new(&universe, Some(constraints.into_iter().collect())).run()
        }
        "python" => {
            Driver::<PythonDAG>::new(&universe, Some(constraints.into_iter().collect())).run()
        }
        _ => panic!("Unknown mode provided"),
    })
}

#[pymodule]
fn aorist(py: Python, m: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "attributes")?;
    attribute(submod)?;
    m.add_submodule(submod)?;

    m.add_class::<InnerApproveAccessSelector>()?;
    m.add_class::<InnerCSVEncoding>()?;
    m.add_class::<InnerORCEncoding>()?;
    m.add_class::<InnerTSVEncoding>()?;
    m.add_class::<InnerDataSet>()?;
    m.add_class::<InnerUniverse>()?;
    m.add_class::<InnerStaticDataTable>()?;
    m.add_class::<InnerUser>()?;
    m.add_class::<InnerUpperSnakeCaseCSVHeader>()?;
    m.add_class::<InnerSingleFileLayout>()?;
    m.add_class::<InnerStaticHiveTableLayout>()?;
    m.add_class::<InnerDailyGranularity>()?;
    m.add_class::<InnerDynamicHiveTableLayout>()?;
    m.add_class::<InnerAlluxioLocation>()?;
    m.add_class::<InnerGCSLocation>()?;
    m.add_class::<InnerMinioLocation>()?;
    m.add_class::<InnerWebLocation>()?;
    m.add_class::<InnerGlobalPermissionsAdmin>()?;
    m.add_class::<InnerRoleBinding>()?;
    m.add_class::<InnerTabularSchema>()?;
    m.add_class::<InnerHiveTableStorage>()?;
    m.add_class::<InnerRemoteStorage>()?;
    m.add_class::<InnerRemoteImportStorageSetup>()?;
    m.add_class::<InnerIdentifierTuple>()?;
    m.add_class::<InnerKeyedStruct>()?;
    m.add_class::<InnerUserGroup>()?;
    m.add_class::<InnerEndpointConfig>()?;
    m.add_class::<InnerAlluxioConfig>()?;
    m.add_class::<InnerMinioConfig>()?;
    m.add_class::<InnerGiteaConfig>()?;
    m.add_class::<InnerRangerConfig>()?;
    m.add_class::<InnerPrestoConfig>()?;
    m.add_class::<InnerGzipCompression>()?;
    m.add_class::<InnerZipCompression>()?;
    m.add_wrapped(wrap_pyfunction!(default_tabular_schema))?;
    m.add_wrapped(wrap_pyfunction!(dag))?;
    Ok(())
}
