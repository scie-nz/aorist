use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::native_python_based_flow::NativePythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{format_code, PythonImport};
use abi_stable::std_types::ROption;
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristUniverse, TPrestoEndpoints};
use pyo3::PyResult;
use serde_json::json;
use std::marker::PhantomData;

pub struct JupyterFlowBuilder<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    _universe: PhantomData<U>,
}
impl<U: AoristUniverse> FlowBuilderBase<U> for JupyterFlowBuilder<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    type T = NativePythonBasedFlow<U>;
    fn new() -> Self {
        Self {
            _universe: PhantomData,
        }
    }
}
impl<U: AoristUniverse> PythonBasedFlowBuilder<U> for JupyterFlowBuilder<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_flow_imports(&self) -> AVec<PythonImport> {
        AVec::new()
    }
    fn build_file(
        &self,
        sources: AVec<(AOption<AString>, AString)>,
        _flow_name: AOption<AString>,
    ) -> PyResult<AString> {
        let cells = json!(sources
            .into_iter()
            .map(|(maybe_comment, block)| {
                let format_block = format_code(block).unwrap().as_str().to_string().replace("\\n", "\n");
                match maybe_comment {
                    AOption(ROption::RSome(comment)) => vec![
                        json!({
                            "cell_type": "markdown",
                            "metadata": json!({}),
                            "source": comment.as_str().to_string().replace("# ", "").replace("#", "# ").replace("\\n", "\r"),
                        }),
                        json!({
                            "cell_type": "code",
                            "execution_count": None as Option<usize>,
                            "metadata": json!({}),
                            "source": format_block,
                            "outputs": Vec::<String>::new(),
                        })
                    ],
                    AOption(ROption::RNone) => vec![json!({
                        "cell_type": "code",
                        "execution_count": None as Option<usize>,
                        "metadata": json!({}),
                        "source": format_block,
                        "outputs": Vec::<String>::new(),
                    })],
                }
            })
            .into_iter()
            .flatten()
            .collect::<AVec<_>>());
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
        Ok(serde_json::to_string_pretty(&notebook)
            .unwrap()
            .as_str()
            .into())
    }
}
