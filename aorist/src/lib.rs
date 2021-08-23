use aorist_attributes::attributes_module;
use aorist_constraint::constraints_module;
use aorist_constraint::*;
use aorist_core::*;
use aorist_primitives::*;
use aorist_util::init_logging;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::BTreeMap;

#[pyfunction(dialect_preferences = "vec![
    Dialect::R(R::new()),
    Dialect::Python(aorist_core::Python::new(vec![])), 
    Dialect::Bash(Bash::new()), 
    Dialect::Presto(Presto::new())
]")]
pub fn dag<'a>(
    universe: PyUniverse,
    constraints: Vec<String>,
    mode: &str,
    programs: BTreeMap<String, Vec<AoristConstraintProgram>>,
    dialect_preferences: Vec<Dialect>,
) -> PyResult<String> {
    universe.compute_uuids();
    let (output, _requirements) = match mode {
        "airflow" => PythonBasedDriver::<
            AoristConstraintBuilder<'a>,
            AirflowFlowBuilder<AoristRef<Universe>>,
            AoristRef<Universe>,
            AoristRef<Concept>,
            ConceptAncestry,
            AoristConstraintProgram,
        >::new(
            universe.inner.clone(),
            constraints.into_iter().collect(),
            programs.into_iter().collect(),
            dialect_preferences,
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "prefect" => PythonBasedDriver::<
            AoristConstraintBuilder<'a>,
            PrefectFlowBuilder<AoristRef<Universe>>,
            AoristRef<Universe>,
            AoristRef<Concept>,
            ConceptAncestry,
            AoristConstraintProgram,
        >::new(
            universe.inner.clone(),
            constraints.into_iter().collect(),
            programs.into_iter().collect(),
            dialect_preferences,
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "python" => PythonBasedDriver::<
            AoristConstraintBuilder<'a>,
            PythonFlowBuilder<AoristRef<Universe>>,
            AoristRef<Universe>,
            AoristRef<Concept>,
            ConceptAncestry,
            AoristConstraintProgram,
        >::new(
            universe.inner.clone(),
            constraints.into_iter().collect(),
            programs.into_iter().collect(),
            dialect_preferences,
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        "jupyter" => PythonBasedDriver::<
            AoristConstraintBuilder<'a>,
            JupyterFlowBuilder<AoristRef<Universe>>,
            AoristRef<Universe>,
            AoristRef<Concept>,
            ConceptAncestry,
            AoristConstraintProgram,
        >::new(
            universe.inner.clone(),
            constraints.into_iter().collect(),
            programs.into_iter().collect(),
            dialect_preferences,
        )
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),
        /*"r" => RBasedDriver::<ConstraintBuilder, RBasedFlowBuilder>::new(&universe, constraints.into_iter().collect())
        .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?
        .run(),*/
        _ => panic!("Unknown mode provided: {}", mode),
    }
    .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
    Ok(output.replace("\\\\", "\\"))
}

#[pymodule]
fn aorist(py: pyo3::prelude::Python, m: &PyModule) -> PyResult<()> {
    init_logging();
    attributes_module(py, m)?;
    constraints_module(py, m)?;
    concept_module(py, m)?;
    endpoints_module(py, m)?;
    dialects_module(py, m)?;
    m.add_wrapped(wrap_pyfunction!(dag))?;
    Ok(())
}
