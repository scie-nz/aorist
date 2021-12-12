use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::flow::flow_builder_input::FlowBuilderInput;
use crate::python::{format_code, PythonFlowBuilderInput, PythonImport, PythonPreamble};
use aorist_ast::AST;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use std::collections::BTreeSet;

impl<C, U> FlowBuilderMaterialize<U> for C
where
    Self: Sized,
    C: PythonBasedFlowBuilder<U>,
    <C as FlowBuilderBase<U>>::T:
        ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    type BuilderInputType = PythonFlowBuilderInput;
    type ErrorType = PyErr;

    fn materialize(
        &self,
        statements_and_preambles: AVec<PythonFlowBuilderInput>,
        flow_name: AOption<AString>,
    ) -> Result<AString, Self::ErrorType> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let ast = PyModule::import(py, "ast").unwrap();
        let astor = PyModule::import(py, "astor").unwrap();

        let flow_imports = self.get_flow_imports().into_iter();

        let preambles: LinkedHashSet<PythonPreamble> = statements_and_preambles
            .iter()
            .map(|x| x.clone().get_preambles().into_iter())
            .flatten()
            .collect();

        let preamble_imports = Self::get_preamble_imports(&preambles);

        let imports = statements_and_preambles
            .iter()
            .map(|x| x.get_imports().clone().into_iter())
            .flatten()
            .chain(flow_imports)
            .chain(preamble_imports)
            .collect::<BTreeSet<_>>();

        let imports_ast: AVec<_> = imports
            .into_iter()
            .map(|x| x.to_python_ast_node(py, ast, 0).unwrap())
            .collect();

        let mut statements_with_ast: AVec<_> = statements_and_preambles
            .into_iter()
            .filter(|x| x.has_statements())
            .collect::<AVec<_>>();

        // ast_value without ancestry => short_name => keys
        let mut literals: LinkedHashMap<AST, LinkedHashMap<AString, AVec<_>>> =
            LinkedHashMap::new();

        for pfbi in statements_with_ast.iter() {
            pfbi.extract_literals(&mut literals);
        }
        let assignments_ast = Self::literals_to_assignments(literals);

        if assignments_ast.len() > 0 {
            statements_with_ast.insert(
                0,
                PythonFlowBuilderInput::new(
                    assignments_ast,
                    LinkedHashSet::new(),
                    BTreeSet::new(),
                    "assignments".into(),
                    Some("Common string literals".into()),
                    None,
                ),
            );
        }

        let augmented_statements: Vec<_> = self
            .augment_statements(statements_with_ast, flow_name.clone())
            .into_iter()
            .collect();
        let content: Vec<(AOption<AString>, Vec<&PyAny>)> =
            vec![(None, imports_ast.into_iter().collect::<Vec<_>>())]
                .into_iter()
                .chain(
                    preambles
                        .into_iter()
                        .map(|x| {
                            (
                                None,
                                x.to_python_ast_nodes(py, ast, 0)
                                    .into_iter()
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
                .chain(
                    augmented_statements
                        .into_iter()
                        .map(|x| {
                            (
                                Some(x.get_block_comment()),
                                x.to_python_ast_nodes(py, ast, 0)
                                    .unwrap()
                                    .into_iter()
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
                .collect();

        let mut sources: AVec<(AOption<AString>, AString)> = AVec::new();

        // This is needed since astor will occasionally forget to add a newline
        for (comment, block) in content {
            let mut lines: AVec<AString> = AVec::new();
            for item in block {
                let module = ast.getattr("Expression")?.call1((item,))?;
                let source: PyResult<_> = astor.getattr("to_source")?.call1((module,));
                if let Err(err) = source {
                    err.print(py);
                    panic!("Exception occurred when running to_source.",);
                }
                let out = source
                    .unwrap()
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .into();
                lines.push(out);
            }
            sources.push((
                comment,
                format_code(
                    lines
                        .iter()
                        .map(|x| x.as_str().to_string())
                        .collect::<AVec<String>>()
                        .join("")
                        .as_str()
                        .into(),
                )?,
            ))
        }
        self.build_file(sources, flow_name)
    }
}

/// Encapsulates all the necessary bits for the construction of a Flow written in
/// Python.
pub trait PythonBasedFlowBuilder<U>: FlowBuilderBase<U>
where
    Self: Sized,
    U: AoristUniverse,
{
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn augment_statements(
        &self,
        statements: AVec<PythonFlowBuilderInput>,
        _flow_name: AOption<AString>,
    ) -> AVec<PythonFlowBuilderInput> {
        statements
    }
    fn get_flow_imports(&self) -> AVec<PythonImport>;

    fn build_file(
        &self,
        sources: AVec<(AOption<AString>, AString)>,
        _flow_name: AOption<AString>,
    ) -> PyResult<AString> {
        format_code(
            sources
                .into_iter()
                .map(|(maybe_comment, block)| match maybe_comment {
                    Some(comment) => format!("# {}\n{}\n", comment, block).to_string(),
                    None => block.as_str().into(),
                })
                .collect::<AVec<String>>()
                .join("")
                .as_str()
                .into(),
        )
    }
}
