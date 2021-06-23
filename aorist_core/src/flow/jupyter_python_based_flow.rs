use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::native_python_based_flow::NativePythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::PythonImport;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde_json::json;
use aorist_primitives::AoristUniverse;
use std::marker::PhantomData;

pub struct JupyterFlowBuilder<U: AoristUniverse> {
    _universe: PhantomData<U>,
}
impl <U: AoristUniverse> FlowBuilderBase<U> for JupyterFlowBuilder<U> {
    type T = NativePythonBasedFlow<U>;
    fn new() -> Self {
        Self {
            _universe: PhantomData
        }
    }
}
impl <U: AoristUniverse> PythonBasedFlowBuilder<U> for JupyterFlowBuilder<U> {
    fn get_flow_imports(&self) -> Vec<PythonImport> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        _py: Python<'a>,
        statements: Vec<(String, Option<String>, Option<String>, Vec<&'a PyAny>)>,
        _ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)> {
        statements
            .into_iter()
            .map(|(name, title, body, code)| {
                (
                    match title {
                        Some(t) => match body {
                            Some(b) => format!(
                                "### {}\n\n{}",
                                t,
                                b.split("\n")
                                    .map(|x| format!("{}", x).to_string())
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            )
                            .to_string(),
                            None => format!("### {}", t).to_string(),
                        },
                        None => format!("### {}", name).to_string(),
                    },
                    code,
                )
            })
            .collect()
    }
    fn build_file(&self, sources: Vec<(Option<String>, String)>) -> PyResult<String> {
        let cells = json!(sources
            .into_iter()
            .map(|(maybe_comment, block)| match maybe_comment {
                Some(comment) => vec![
                    json!({
                        "cell_type": "markdown",
                        "metadata": json!({}),
                        "source": comment,
                    }),
                    json!({
                        "cell_type": "code",
                        "execution_count": None as Option<usize>,
                        "metadata": json!({}),
                        "source": block,
                        "outputs": Vec::<String>::new(),
                    })
                ],
                None => vec![json!({
                    "cell_type": "code",
                    "execution_count": None as Option<usize>,
                    "metadata": json!({}),
                    "source": block,
                    "outputs": Vec::<String>::new(),

                })],
            })
            .into_iter()
            .flatten()
            .collect::<Vec<_>>());
        let notebook = json!({
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
        });
        Ok(serde_json::to_string_pretty(&notebook).unwrap())
    }
}
