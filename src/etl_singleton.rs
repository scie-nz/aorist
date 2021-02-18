use crate::endpoints::EndpointConfig;
use crate::python::{format_code, Import, Preamble, PythonStatementInput, AST};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString};
use std::collections::BTreeSet;

pub trait ETLSingleton {
    fn get_preamble(&self) -> Vec<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> AST;
    fn new(
        task_id: AST,
        // TODO: change this to optional dict
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        endpoints: EndpointConfig,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_type() -> String;
    fn get_imports(&self) -> Vec<Import>;
}
pub trait ETLDAG
where
    Self: Sized,
{
    type T: ETLSingleton;

    fn new() -> Self;
    fn build_flow<'a>(
        &self,
        py: Python<'a>,
        statements: Vec<(String, Option<String>, Option<String>, Vec<&'a PyAny>)>,
        ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)>;
    fn get_flow_imports(&self) -> Vec<Import>;

    fn get_preamble_imports(&self, preambles: &LinkedHashSet<Preamble>) -> Vec<Import> {
        let preamble_module_imports = preambles
            .iter()
            .map(|x| x.imports.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<_>>();

        // TODO: group imports by module (need to change def of import)
        let from_imports = preambles
            .iter()
            .map(|x| x.from_imports.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<_>>();
        let preamble_imports = preamble_module_imports
            .into_iter()
            .chain(from_imports.into_iter())
            .collect::<Vec<_>>();
        preamble_imports
    }
    fn materialize(&self, statements_and_preambles: Vec<PythonStatementInput>) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let ast = PyModule::import(py, "ast").unwrap();
        let astor = PyModule::import(py, "astor").unwrap();

        let flow_imports = self.get_flow_imports().into_iter();

        let preambles = statements_and_preambles
            .iter()
            .map(|x| x.clone().1.into_iter())
            .flatten()
            .collect::<LinkedHashSet<Preamble>>();

        let preamble_imports: Vec<Import> = self.get_preamble_imports(&preambles);

        let imports = statements_and_preambles
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .chain(flow_imports)
            .chain(preamble_imports)
            .collect::<BTreeSet<Import>>();

        let imports_ast: Vec<_> = imports
            .into_iter()
            .map(|x| x.to_python_ast_node(py, ast).unwrap())
            .collect();

        let statements: Vec<(String, Option<String>, Option<String>, Vec<AST>)> =
            statements_and_preambles
                .into_iter()
                .map(|x| (x.3, x.4, x.5, x.0))
                .collect();
        let statements_ast: Vec<_> = statements
            .into_iter()
            .filter(|x| x.3.len() > 0)
            .map(|(name, title, body, x)| {
                (
                    name,
                    title,
                    body,
                    x.into_iter()
                        .map(|y| y.to_python_ast_node(py, ast).unwrap())
                        .collect(),
                )
            })
            .collect();

        let flow = self.build_flow(py, statements_ast, ast);

        let content: Vec<(String, Vec<&PyAny>)> =
            vec![("Imports".to_string(), imports_ast)]
                .into_iter()
                .chain(
                    preambles
                        .into_iter()
                        .enumerate()
                        .map(|(i, x)| (format!("Preamble {}", i).to_string(), x.get_body_ast(py))),
                )
                .chain(flow.into_iter())
                .collect();

        let mut sources: Vec<(String, String)> = Vec::new();

        // This is needed since astor will occasionally forget to add a newline
        for (comment, block) in content {
            let mut lines: Vec<String> = Vec::new();
            for item in block {
                let statements_list = PyList::new(py, vec![item]);
                let module = ast.call1("Module", (statements_list,))?;
                let source: PyResult<_> = astor.call1("to_source", (module,));
                if let Err(err) = source {
                    err.print(py);
                    panic!("Exception occurred when running to_source.");
                }
                lines.push(
                    source
                        .unwrap()
                        .extract::<&PyString>()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            }
            sources.push((comment, format_code(lines.join(""))?))
        }
        self.build_file(sources)
    }
    fn build_file(&self, sources: Vec<(String, String)>) -> PyResult<String> {
        format_code(
            sources
                .into_iter()
                .map(|(comment, block)| format!("# {}\n{}\n", comment, block).to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
