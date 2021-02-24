use crate::constraint_state::AncestorRecord;
use crate::endpoints::EndpointConfig;
use crate::python::{
    format_code, Assignment, Import, Preamble, PythonStatementInput, SimpleIdentifier,
    StringLiteral, AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyString};
use std::collections::{BTreeMap, BTreeSet};

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
            .map(|x| x.to_python_ast_node(py, ast, 0).unwrap())
            .collect();

        let statements: Vec<(String, Option<String>, Option<String>, Vec<AST>)> =
            statements_and_preambles
                .into_iter()
                .map(|x| (x.3, x.4, x.5, x.0))
                .collect();
        let mut statements_with_ast: Vec<_> = statements
            .into_iter()
            .filter(|x| x.3.len() > 0)
            .collect::<Vec<_>>();

        // ast_value without ancestry => short_name => keys
        let mut literals: LinkedHashMap<AST, LinkedHashMap<String, Vec<String>>> =
            LinkedHashMap::new();

        for (short_name, _, _, asts) in statements_with_ast.iter() {
            for ast in asts {
                if let AST::Assignment(rw) = ast {
                    let assign = rw.read().unwrap();
                    if let AST::Dict(dict_rw) = assign.call() {
                        let dict = dict_rw.read().unwrap();
                        for (task_key, task_val) in dict.elems() {
                            if let AST::Dict(param_dict_rw) = task_val {
                                let param_dict = param_dict_rw.read().unwrap();
                                for (key, val) in param_dict.elems() {
                                    if let Some(ancestors) = val.get_ancestors() {
                                        literals
                                            .entry(val.clone_without_ancestors())
                                            .or_insert(LinkedHashMap::new())
                                            .entry(short_name.to_string())
                                            .or_insert(Vec::new())
                                            .push(key.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut assignments = LinkedHashMap::new();
        for (literal, val) in literals.into_iter() {
            for (short_name, keys) in val.into_iter() {
                let mut keys_hist: BTreeMap<String, usize> = BTreeMap::new();
                for key in keys {
                    *keys_hist.entry(key).or_insert(1) += 1;
                }
                if keys_hist.len() == 1 {
                    assignments
                        .entry(
                            format!(
                                "{}__{}",
                                keys_hist.into_iter().next().unwrap().0,
                                short_name
                            )
                            .to_string(),
                        )
                        .or_insert(Vec::new())
                        .push(literal.clone());
                }
            }
        }
        let assignments_ast = assignments
            .into_iter()
            .filter(|(_, vals)| vals.len() == 1)
            .map(|(var, vals)| {
                let lval = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(var));
                let rval = vals.into_iter().next().unwrap();
                AST::Assignment(Assignment::new_wrapped(lval, rval))
            })
            .collect::<Vec<_>>();
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

        let content: Vec<(String, Vec<&PyAny>)> = vec![("Imports".to_string(), imports_ast)]
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
                //let statements_list = PyList::new(py, vec![item]);
                let module = ast.call1("Expression", (item,))?;
                let source: PyResult<_> = astor.call1("to_source", (module,));
                if let Err(err) = source {
                    err.print(py);
                    panic!("Exception occurred when running to_source.",);
                }
                //let dump: PyResult<_> = ast.call1("dump", (module,));
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
