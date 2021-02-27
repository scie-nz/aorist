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
pub mod driver;
pub mod encoding;
pub mod endpoints;
pub mod error;
pub mod etl_singleton;
pub mod etl_task;
pub mod features;
pub mod header;
pub mod jupyter_singleton;
pub mod layout;
pub mod location;
pub mod models;
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
pub use storage::*;
pub use storage_setup::*;
pub use template::*;
pub use user::*;
pub use user_group::*;
pub use utils::*;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashSet;

use pyo3::create_exception;
use pyo3::exceptions::PyException;
create_exception!(aorist, SQLParseError, PyException);

#[pyfunction]
pub fn derived_asset(sql: String, universe: &InnerUniverse) -> PyResult<()> {
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, &sql).unwrap();
    if ast.len() != 1 {
        return Err(SQLParseError::new_err(
            "A single SELECT statement should be provided.",
        ));
    }
    println!("AST: {:?}", &ast);
    if let Statement::Query(query) = ast.into_iter().next().unwrap() {
    } else {
        return Err(SQLParseError::new_err(
            "Only SELECT statements supported.",
        ));
    }
    Ok(())
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
    let template_name: String = source_asset.get_schema().get_datum_template_name();
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

#[pyfunction]
pub fn dag(inner: InnerUniverse, constraints: Vec<String>, mode: &str) -> PyResult<String> {
    let mut universe = Universe::from(inner);
    universe.compute_uuids();
    let debug = false;
    match mode {
        "airflow" => {
            Driver::<AirflowDAG>::new(&universe, constraints.into_iter().collect(), debug).run()
        }
        "prefect" => {
            Driver::<PrefectDAG>::new(&universe, constraints.into_iter().collect(), debug).run()
        }
        "python" => {
            Driver::<PythonDAG>::new(&universe, constraints.into_iter().collect(), debug).run()
        }
        "jupyter" => {
            Driver::<JupyterDAG>::new(&universe, constraints.into_iter().collect(), debug).run()
        }
        _ => panic!("Unknown mode provided"),
    }
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
    m.add_wrapped(wrap_pyfunction!(default_tabular_schema))?;
    m.add_wrapped(wrap_pyfunction!(dag))?;
    m.add_wrapped(wrap_pyfunction!(derive_integer_measure))?;
    m.add_wrapped(wrap_pyfunction!(derived_asset))?;
    m.add("SQLParseError", py.get_type::<SQLParseError>())?;
    Ok(())
}
