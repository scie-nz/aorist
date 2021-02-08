use crate::python::{AoristStatement, Import, Preamble, PythonStatementInput, AST};
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
    ) -> Self;
    fn compute_task_call(&self) -> AST;
    fn compute_task_args(&self) -> Vec<AST>;
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST>;
    fn get_statements(&self) -> Vec<AoristStatement>;
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
        statements: Vec<&'a PyAny>,
        ast_module: &'a PyModule,
    ) -> Vec<&'a PyAny>;
    fn get_flow_imports(&self) -> Vec<Import>;

    fn get_preamble_imports(&self, preambles: &Vec<Preamble>) -> Vec<Import> {
        let preamble_module_imports = preambles
            .iter()
            .map(|x| {
                x.imports
                    .clone()
                    .into_iter()
                    .map(|x| Import::ModuleImport(x.0))
            })
            .flatten()
            .collect::<BTreeSet<_>>();

        // TODO: group imports by module (need to change def of import)
        let mut from_imports: BTreeSet<_> = BTreeSet::new();
        let preamble_from_imports = preambles
            .iter()
            .map(|x| x.from_imports.clone().into_iter())
            .flatten();
        for (module, name, _alias) in preamble_from_imports {
            from_imports.insert(Import::FromImport(module, name));
        }
        let preamble_imports = preamble_module_imports
            .into_iter()
            .chain(from_imports.into_iter())
            .collect::<Vec<_>>();
        preamble_imports
    }
    fn materialize(&self, statements_and_preambles: Vec<PythonStatementInput>) -> String {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let ast = PyModule::import(py, "ast").unwrap();
        let astor = PyModule::import(py, "astor").unwrap();

        let flow_imports = self.get_flow_imports().into_iter();

        let preambles = statements_and_preambles
            .iter()
            .map(|x| x.clone().1.into_iter())
            .flatten()
            .collect::<LinkedHashSet<String>>();

        let processed_preambles = preambles
            .into_iter()
            .map(|x| Preamble::new(x, py))
            .collect::<Vec<Preamble>>();

        let preamble_imports: Vec<Import> = self.get_preamble_imports(&processed_preambles);

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

        let statements: Vec<AoristStatement> = statements_and_preambles
            .into_iter()
            .map(|x| x.0)
            .flatten()
            .collect();
        let statements_ast: Vec<_> = statements
            .into_iter()
            .map(|x| x.to_python_ast_node(py, ast).unwrap())
            .collect();
        let flow: Vec<&PyAny> = self.build_flow(py, statements_ast, ast);

        let content: Vec<&PyAny> = imports_ast
            .into_iter()
            .chain(processed_preambles.into_iter().map(|x| x.body).flatten())
            .chain(flow.into_iter())
            .collect();

        let statements_list = PyList::new(py, content);
        let module = ast.call1("Module", (statements_list,)).unwrap();
        let source: PyResult<_> = astor.call1("to_source", (module,));
        if let Err(err) = source {
            err.print(py);
            panic!("Exception occurred when running to_source.");
        }
        source
            .unwrap()
            .extract::<&PyString>()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
