use crate::python::{
    AoristStatement, ArgType, Import, Preamble, PythonProgram, PythonStatementInput,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use rustpython_parser::ast::{Located, Location, Statement, StatementType};
use std::collections::{BTreeMap, BTreeSet};

pub trait ETLSingleton {
    fn get_preamble(&self) -> Vec<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> ArgType;
    fn new(
        task_id: ArgType,
        // TODO: change this to optional dict
        task_val: ArgType,
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self;
    fn compute_task_call(&self) -> ArgType;
    fn compute_task_args(&self) -> Vec<ArgType>;
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType>;
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
    fn build_flow(&self, statements: Vec<Statement>, _location: Location) -> Vec<Statement>;
    fn get_flow_imports(&self) -> Vec<Import>;

    fn materialize(&self, statements_and_preambles: Vec<PythonStatementInput>) -> String {
        let flow_imports = self.get_flow_imports().into_iter();
        let location = Location::new(0, 0);
        let preambles = statements_and_preambles
            .iter()
            .map(|x| x.clone().1.into_iter())
            .flatten()
            .collect::<LinkedHashSet<String>>();

        let processed_preambles = preambles
            .into_iter()
            .map(|x| Preamble::new(x))
            .collect::<Vec<_>>();

        let preamble_module_imports = processed_preambles
            .iter()
            .map(|x| x.imports.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<_>>();

        let mut from_imports: BTreeMap<Option<String>, BTreeSet<_>> = BTreeMap::new();
        let preamble_from_imports = processed_preambles
            .iter()
            .map(|x| x.from_imports.clone().into_iter())
            .flatten();
        for (module, names) in preamble_from_imports {
            for name in names {
                from_imports
                    .entry(module.clone())
                    .or_insert(BTreeSet::new())
                    .insert(name);
            }
        }
        let preamble_imports = preamble_module_imports
            .into_iter()
            .map(|x| Located {
                location,
                node: StatementType::Import { names: vec![x.0] },
            })
            .chain(from_imports.into_iter().map(|(module, names)| Located {
                location,
                node: StatementType::ImportFrom {
                    level: 0,
                    module,
                    names: names.into_iter().map(|x| x.0).collect(),
                },
            }))
            .collect::<Vec<_>>();

        let imports = statements_and_preambles
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .chain(flow_imports)
            .collect::<BTreeSet<Import>>();
        let statements: Vec<AoristStatement> = statements_and_preambles
            .into_iter()
            .map(|x| x.0)
            .flatten()
            .collect();
        let content = PythonProgram::render_suite(
            preamble_imports
                .into_iter()
                .chain(
                    imports
                        .into_iter()
                        .map(|x| AoristStatement::Import(x).statement(location)),
                )
                .chain(processed_preambles.into_iter().map(|x| x.statement()))
                .chain(
                    self.build_flow(
                        statements
                            .into_iter()
                            .map(|x| x.statement(location))
                            .collect(),
                        location,
                    )
                    .into_iter(),
                )
                .collect(),
        )
        .unwrap();
        content
    }
}
