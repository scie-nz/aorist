use crate::code::Preamble;
use crate::flow::flow_builder::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::flow::flow_builder_input::FlowBuilderInput;
use crate::python::{format_code, PythonFlowBuilderInput, PythonImport, PythonPreamble, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use std::collections::BTreeSet;

impl<C> FlowBuilderMaterialize for C
where
    Self: Sized,
    C: PythonBasedFlowBuilder,
{
    type BuilderInputType = PythonFlowBuilderInput;
    type ErrorType = PyErr;

    fn materialize(
        &self,
        statements_and_preambles: Vec<PythonFlowBuilderInput>,
    ) -> Result<String, Self::ErrorType> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let ast = PyModule::import(py, "ast").unwrap();
        let astor = PyModule::import(py, "astor").unwrap();

        let flow_imports = self.get_flow_imports().into_iter();

        let preambles = statements_and_preambles
            .iter()
            .map(|x| x.clone().get_preambles().into_iter())
            .flatten()
            .collect::<LinkedHashSet<PythonPreamble>>();

        let preamble_imports: Vec<PythonImport> = self.get_preamble_imports(&preambles);

        let imports = statements_and_preambles
            .iter()
            .map(|x| x.get_imports().clone().into_iter())
            .flatten()
            .chain(flow_imports)
            .chain(preamble_imports)
            .collect::<BTreeSet<PythonImport>>();

        let imports_ast: Vec<_> = imports
            .into_iter()
            .map(|x| x.to_python_ast_node(py, ast, 0).unwrap())
            .collect();

        let statements: Vec<(String, Option<String>, Option<String>, Vec<AST>)> =
            statements_and_preambles
                .into_iter()
                .map(|x| {
                    (
                        x.get_constraint_name(),
                        x.get_constraint_title(),
                        x.get_constraint_body(),
                        x.get_statements(),
                    )
                })
                .collect();
        let mut statements_with_ast: Vec<_> = statements
            .into_iter()
            .filter(|x| x.3.len() > 0)
            .collect::<Vec<_>>();

        // ast_value without ancestry => short_name => keys
        let mut literals: LinkedHashMap<AST, LinkedHashMap<String, Vec<_>>> = LinkedHashMap::new();

        for (short_name, _, _, asts) in statements_with_ast.iter() {
            for ast in asts {
                Self::extract_literals(ast, &short_name, &mut literals);
            }
        }
        let assignments_ast = Self::literals_to_assignments(literals);

        if assignments_ast.len() > 0 {
            statements_with_ast.insert(
                0,
                (
                    "assignments".to_string(),
                    Some("Common string literals".to_string()),
                    None,
                    assignments_ast,
                ),
            );
        }
        let statements_ast = statements_with_ast
            .into_iter()
            .map(|(name, title, body, x)| {
                (
                    name,
                    title,
                    body,
                    x.into_iter()
                        .map(|y| y.to_python_ast_node(py, ast, 0).unwrap())
                        .collect(),
                )
            })
            .collect();

        let flow = self.build_flow(py, statements_ast, ast);

        let content: Vec<(Option<String>, Vec<&PyAny>)> =
            vec![(Some("PythonImports".to_string()), imports_ast)]
                .into_iter()
                .chain(preambles.into_iter().map(|x| (None, x.get_body_ast(py))))
                .chain(flow.into_iter().map(|(x, y)| (Some(x), y)))
                .collect();

        let mut sources: Vec<(Option<String>, String)> = Vec::new();

        // This is needed since astor will occasionally forget to add a newline
        for (comment, block) in content {
            let mut lines: Vec<String> = Vec::new();
            for item in block {
                //let statements_list = PyList::new(py, vec![item]);
                let module = ast.call1("Expression", (item,))?;
                let source: PyResult<_> = astor.call1("to_source", (module,));
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
                    .to_string();
                lines.push(out);
            }
            sources.push((comment, format_code(lines.join(""))?))
        }
        self.build_file(sources)
    }
}

/// Encapsulates all the necessary bits for the construction of a Flow written in
/// Python.
pub trait PythonBasedFlowBuilder: FlowBuilderBase
where
    Self: Sized,
{
    fn build_flow<'a>(
        &self,
        py: Python<'a>,
        statements: Vec<(String, Option<String>, Option<String>, Vec<&'a PyAny>)>,
        ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)>;
    fn get_flow_imports(&self) -> Vec<PythonImport>;

    fn get_preamble_imports(&self, preambles: &LinkedHashSet<PythonPreamble>) -> Vec<PythonImport> {
        preambles
            .iter()
            .map(|x| x.get_imports().into_iter())
            .flatten()
            .collect()
    }
    fn build_file(&self, sources: Vec<(Option<String>, String)>) -> PyResult<String> {
        format_code(
            sources
                .into_iter()
                .map(|(maybe_comment, block)| match maybe_comment {
                    Some(comment) => format!("# {}\n{}\n", comment, block).to_string(),
                    None => block,
                })
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
