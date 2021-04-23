#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

extern crate pyo3;
pub mod access_policy;
pub mod airflow_singleton;
pub mod algorithms;
pub mod asset;
pub mod attributes;
pub mod code_block;
pub mod compliance;
pub mod compression;
pub mod concept;
pub mod constraint;
pub mod constraint_block;
pub mod constraint_state;
pub mod data_setup;
pub mod dataset;
pub mod dialect;
pub mod driver;
pub mod encoding;
pub mod endpoints;
pub mod error;
pub mod etl_flow;
pub mod features;
pub mod header;
pub mod jupyter_singleton;
pub mod layout;
pub mod location;
pub mod logging;
pub mod models;
pub mod object;
pub mod prefect_singleton;
pub mod python;
pub mod python_based_dag;
pub mod python_based_task;
pub mod python_singleton;
pub mod role;
pub mod role_binding;
pub mod schema;
pub mod sql_parser;
pub mod storage;
pub mod storage_setup;
pub mod template;
pub mod user;
pub mod user_group;
pub mod utils;

pub use access_policy::*;
pub use airflow_singleton::*;
pub use algorithms::*;
pub use asset::*;
pub use attributes::*;
pub use compliance::*;
pub use compression::*;
pub use concept::*;
pub use data_setup::*;
pub use dataset::*;
pub use driver::*;
pub use encoding::*;
pub use endpoints::*;
pub use features::*;
pub use header::*;
pub use jupyter_singleton::*;
pub use layout::*;
pub use location::*;
pub use models::*;
pub use prefect_singleton::*;
pub use python_singleton::*;
pub use role::*;
pub use role_binding::*;
pub use schema::*;
pub use sql_parser::*;
pub use storage::*;
pub use storage_setup::*;
pub use template::*;
pub use user::*;
pub use user_group::*;
pub use utils::*;

use anyhow::bail;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashSet;

#[pyfunction]
pub fn default_time_ordered_tabular_schema(
    datum_template: InnerDatumTemplate,
    ordering_attributes: Vec<String>,
) -> InnerTimeOrderedTabularSchema {
    InnerTimeOrderedTabularSchema {
        datumTemplateName: datum_template.get_name(),
        attributes: DatumTemplate::from(datum_template)
            .get_attributes()
            .iter()
            .map(|x| x.get_name().clone())
            .collect(),
        tag: None,
        orderingAttributes: ordering_attributes,
    }
}
#[pyfunction]
pub fn default_tabular_schema(datum_template: InnerDatumTemplate) -> InnerTabularSchema {
    InnerTabularSchema {
        datumTemplateName: datum_template.get_name(),
        attributes: DatumTemplate::from(datum_template)
            .get_attributes()
            .iter()
            .map(|x| x.get_name().clone())
            .collect(),
        tag: None,
    }
}

#[pyfunction]
pub fn derive_integer_measure(
    dataset: InnerDataSet,
    source_asset: InnerAsset,
    name: String,
    comment: Option<String>,
    attribute_names: Vec<String>,
) -> InnerIntegerMeasure {
    let keep: HashSet<String> = attribute_names.into_iter().collect();
    let template_name: String = source_asset.get_schema().get_datum_template_name().unwrap();
    let attribute_names: HashSet<String> = source_asset
        .get_schema()
        .get_attribute_names()
        .into_iter()
        .collect();
    for name in &keep {
        if !attribute_names.contains(name) {
            panic!("Could not find attribute {} in the schema.", name);
        }
    }
    if let Some(template) = dataset.get_mapped_datum_templates().get(&template_name) {
        let attributes = DatumTemplate::from(template.clone())
            .get_attributes()
            .into_iter()
            .filter(|x| keep.contains(x.get_name()))
            .map(|x| InnerAttribute::from(x.clone()))
            .collect::<Vec<InnerAttribute>>();

        return InnerIntegerMeasure {
            attributes,
            name,
            comment,
            source_asset_name: source_asset.get_name(),
            tag: None,
        };
    } else {
        panic!("Could not find template {} in templates.", template_name);
    }
}

// TODO: these two functions should not require the round-trip to Universe
#[pyfunction]
pub fn serialize(inner: InnerUniverse) -> PyResult<String> {
    let universe = Universe::from(inner);
    let s = serde_yaml::to_string(&universe).unwrap();
    Ok(s)
}
#[pyfunction]
pub fn deserialize(input: String) -> PyResult<InnerUniverse> {
    let universe: Universe = serde_yaml::from_str(&input).unwrap();
    Ok(universe.into())
}

#[pyfunction]
pub fn dag(inner: InnerUniverse, constraints: Vec<String>, mode: &str) -> PyResult<String> {
    let mut universe = Universe::from(inner);
    universe.compute_uuids();
    let (output, _requirements) = match mode {
        "airflow" => Driver::<AirflowDAG>::new(&universe, constraints.into_iter().collect())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
            .run(),
        "prefect" => Driver::<PrefectDAG>::new(&universe, constraints.into_iter().collect())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
            .run(),
        "python" => Driver::<PythonDAG>::new(&universe, constraints.into_iter().collect())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
            .run(),
        "jupyter" => Driver::<JupyterDAG>::new(&universe, constraints.into_iter().collect())
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
            .run(),
        _ => panic!("Unknown mode provided: {}", mode),
    }
    .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
    Ok(output.replace("\\\\", "\\"))
}

#[pyfunction]
pub fn attr_list(input: Vec<AttributeEnum>) -> PyResult<Vec<InnerAttribute>> {
    Ok(input
        .into_iter()
        .map(|x| InnerAttribute {
            inner: AttributeOrTransform::Attribute(x),
            tag: None,
        })
        .collect())
}

#[pymodule]
fn aorist(py: Python, m: &PyModule) -> PyResult<()> {
    logging::init_logging();

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
    m.add_class::<InnerCSVHeader>()?;
    m.add_class::<InnerSingleFileLayout>()?;
    m.add_class::<InnerStaticTabularLayout>()?;
    m.add_class::<InnerDailyGranularity>()?;
    m.add_class::<InnerDynamicTabularLayout>()?;
    m.add_class::<InnerAlluxioLocation>()?;
    m.add_class::<InnerGCSLocation>()?;
    m.add_class::<InnerMinioLocation>()?;
    m.add_class::<InnerWebLocation>()?;
    m.add_class::<InnerGlobalPermissionsAdmin>()?;
    m.add_class::<InnerRoleBinding>()?;
    m.add_class::<InnerTabularSchema>()?;
    m.add_class::<InnerTimeOrderedTabularSchema>()?;
    m.add_class::<InnerHiveTableStorage>()?;
    m.add_class::<InnerRemoteStorage>()?;
    m.add_class::<InnerReplicationStorageSetup>()?;
    m.add_class::<InnerIdentifierTuple>()?;
    m.add_class::<InnerRowStruct>()?;
    m.add_class::<InnerUserGroup>()?;
    m.add_class::<InnerEndpointConfig>()?;
    m.add_class::<InnerAlluxioConfig>()?;
    m.add_class::<InnerMinioConfig>()?;
    m.add_class::<InnerGiteaConfig>()?;
    m.add_class::<InnerRangerConfig>()?;
    m.add_class::<InnerPrestoConfig>()?;
    m.add_class::<InnerGzipCompression>()?;
    m.add_class::<InnerZipCompression>()?;
    m.add_class::<InnerComplianceConfig>()?;
    m.add_class::<InnerSingleObjectiveRegressor>()?;
    m.add_class::<InnerRandomForestRegressionAlgorithm>()?;
    m.add_class::<InnerSVMRegressionAlgorithm>()?;
    m.add_class::<InnerContinuousObjective>()?;
    m.add_class::<InnerComputedFromLocalData>()?;
    m.add_class::<InnerIntegerMeasure>()?;
    m.add_class::<InnerTrainedFloatMeasure>()?;
    m.add_class::<InnerSupervisedModel>()?;
    m.add_class::<InnerFilter>()?;
    m.add_class::<InnerONNXEncoding>()?;
    m.add_class::<InnerLocalFileStorage>()?;
    m.add_class::<InnerRemoteStorageSetup>()?;
    m.add_class::<InnerPushshiftAPILocation>()?;
    m.add_class::<InnerPushshiftSubredditPostsAPILayout>()?;
    m.add_class::<InnerJSONEncoding>()?;
    m.add_class::<InnerSQLiteLocation>()?;
    m.add_class::<InnerSQLiteStorage>()?;
    m.add_class::<InnerPostgresLocation>()?;
    m.add_class::<InnerPostgresConfig>()?;
    m.add_class::<InnerPostgresStorage>()?;
    m.add_class::<InnerGCPConfig>()?;
    m.add_class::<InnerBigQueryLocation>()?;
    m.add_class::<InnerBigQueryStorage>()?;
    m.add_class::<InnerLocalStorageSetup>()?;
    m.add_class::<InnerUndefinedTabularSchema>()?;
    m.add_class::<InnerLocalFileSystemLocation>()?;
    m.add_class::<InnerNewlineDelimitedJSONEncoding>()?;
    m.add_class::<InnerGithubLocation>()?;
    m.add_class::<InnerGitStorage>()?;
    m.add_wrapped(wrap_pyfunction!(default_tabular_schema))?;
    m.add_wrapped(wrap_pyfunction!(dag))?;
    m.add_wrapped(wrap_pyfunction!(derive_integer_measure))?;
    m.add_wrapped(wrap_pyfunction!(attr_list))?;
    m.add_wrapped(wrap_pyfunction!(default_time_ordered_tabular_schema))?;
    m.add("SQLParseError", py.get_type::<SQLParseError>())?;
    Ok(())
}
