use crate::etl_singleton::ETLDAG;
use crate::python::Import;
use crate::python_singleton::PythonSingleton;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde_json::json;

pub struct JupyterDAG {}
impl ETLDAG for JupyterDAG {
    type T = PythonSingleton;
    fn new() -> Self {
        Self {}
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        _py: Python<'a>,
        statements: Vec<(String, Vec<&'a PyAny>)>,
        _ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)> {
        statements
    }
    fn build_file(&self, sources: Vec<(String, String)>) -> PyResult<String> {
        let cells = json!(sources
            .into_iter()
            .map(|(comment, block)| vec![
                json!({
                    "cell_type": "markdown",
                    "metadata": json!({}),
                    "source": format!("# {}", comment),
                }),
                json!({
                    "cell_type": "code",
                    "execution_count": None as Option<usize>,
                    "metadata": json!({}),
                    "source": block,
                    "outputs": Vec::<String>::new(),

                })
            ]
            .into_iter())
            .flatten()
            .collect::<Vec<_>>());
        Ok(serde_json::to_string_pretty(&json!({
            "nbformat": 4,
            "nbformat_minor": 5,
            "cells": cells,
           "metadata": json!({
            "kernelspec": json!({
             "display_name": "Python 3",
             "language": "python",
             "name": "python3"
            }),
            "language_info": json!({
             "codemirror_mode": json!({
              "name": "ipython",
              "version": 3
             }),
             "file_extension": ".py",
             "mimetype": "text/x-python",
             "name": "python",
             "nbconvert_exporter": "python",
             "pygments_lexer": "ipython3",
             "version": "3.8.5"
            })
           }),
        })
        ).unwrap())
    }
}
