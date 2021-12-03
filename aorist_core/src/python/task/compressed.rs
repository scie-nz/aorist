use crate::flow::{CompressionKey, ETLFlow, ForLoopCompressedTask, TaskBase, UncompressiblePart};
use crate::python::task::key::PythonBasedTaskCompressionKey;
use crate::python::task::uncompressible::PythonBasedTaskUncompressiblePart;
use crate::python::{
    Add, Assignment, Attribute, BigIntLiteral, BinOp, Call, Dict, ForLoop, List, PythonImport,
    PythonPreamble, SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
use aorist_primitives::AoristUniverse;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use tracing::trace;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ForLoopPythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport>,
    U: AoristUniverse,
{
    params_dict_name: AST,
    key: PythonBasedTaskCompressionKey,
    values: Vec<PythonBasedTaskUncompressiblePart<T, U>>,
    singleton_type: PhantomData<T>,
    task_id: AST,
    insert_task_name: bool,
    _universe: PhantomData<U>,
    render_dependencies: bool,
}
impl<T, U> ForLoopCompressedTask<T, U> for ForLoopPythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    type KeyType = PythonBasedTaskCompressionKey;
    type UncompressiblePartType = PythonBasedTaskUncompressiblePart<T, U>;
    fn new(
        params_dict_name: AST,
        key: PythonBasedTaskCompressionKey,
        values: Vec<PythonBasedTaskUncompressiblePart<T, U>>,
        task_id: AST,
        insert_task_name: bool,
        render_dependencies: bool,
    ) -> Self {
        trace!("New compressed task with key: {:?}", key);
        trace!("uncompressible:");
        for v in &values {
            trace!("-- {:?} : {:?}", v.dict, v.params);
        }
        let distinct_keys = values
            .iter()
            .map(|x| x.dict.clone())
            .collect::<std::collections::HashSet<_>>();
        if distinct_keys.len() < values.len() {
            panic!("Tasks with same keys in for loop compression.");
        }
        Self {
            params_dict_name,
            key,
            values,
            task_id,
            insert_task_name,
            singleton_type: PhantomData,
            _universe: PhantomData,
            render_dependencies,
        }
    }
}
impl<T, U> TaskBase<T, U> for ForLoopPythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
}

impl<T, U> ForLoopPythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    fn get_dict_assign(&self) -> (AST, bool) {
        let insert_deps = self.render_dependencies
            && self
                .values
                .iter()
                .filter(|x| x.deps.len() > 0)
                .next()
                .is_some();
        // true if there is no task with but a single dependency
        let dependencies_as_list = !self
            .values
            .iter()
            .filter(|x| x.deps.len() != 1)
            .next()
            .is_none();
        let dict_pairs = self
            .values
            .iter()
            .map(|x| {
                (
                    x.dict.clone(),
                    x.as_dict(insert_deps, dependencies_as_list, self.insert_task_name),
                )
            })
            .collect::<LinkedHashMap<_, _>>();
        let has_params_dict = dict_pairs
            .iter()
            .filter(|x| {
                if let AST::Dict(ref dict) = x.1 {
                    dict.read().len() > 0
                } else {
                    panic!("This should be a dictionary.");
                }
            })
            .next()
            .is_some();

        let dict_content = match has_params_dict {
            true => AST::Dict(Dict::new_wrapped(dict_pairs)),
            false => AST::List(List::new_wrapped(
                dict_pairs
                    .into_iter()
                    .map(|x| AST::StringLiteral(StringLiteral::new_wrapped(x.0, false)))
                    .collect(),
                false,
            )),
        };
        (
            AST::Assignment(Assignment::new_wrapped(
                self.params_dict_name.clone(),
                dict_content,
            )),
            has_params_dict,
        )
    }
    fn get_for_loop_tuple(&self, ident: &AST, params: &AST) -> AST {
        AST::Tuple(Tuple::new_wrapped(
            vec![ident.clone(), params.clone()],
            false,
        ))
    }
    fn get_task_collector(&self, ident: &AST) -> AST {
        AST::Subscript(Subscript::new_wrapped(
            self.key.clone().get_dict_name(),
            ident.clone(),
            false,
        ))
    }
    pub fn get_statements(
        &self,
        endpoints: U::TEndpoints,
    ) -> (Vec<AST>, Vec<PythonPreamble>, Vec<PythonImport>) {
        let any_dependencies = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 0)
            .next()
            .is_some();

        let (dict_assign, has_params_dict) = self.get_dict_assign();

        let params = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".to_string()));
        let ident = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".to_string()));

        let tpl = match has_params_dict {
            true => self.get_for_loop_tuple(&ident, &params),
            false => ident.clone(),
        };
        let new_collector = self.get_task_collector(&ident);

        let mut kwargs;
        let args;
        if let Some((num_args, kwarg_keys)) = self.key.get_dedup_key() {
            kwargs = kwarg_keys
                .iter()
                .map(|x| {
                    (
                        x.clone(),
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(x.to_string(), false)),
                            false,
                        )),
                    )
                })
                .collect::<LinkedHashMap<_, _>>();

            args = (0..num_args)
                .map(|x| {
                    AST::Subscript(Subscript::new_wrapped(
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(
                                "args".to_string(),
                                false,
                            )),
                            false,
                        )),
                        AST::BigIntLiteral(BigIntLiteral::new_wrapped(x as i64)),
                        false,
                    ))
                })
                .collect::<Vec<AST>>();
        } else {
            kwargs = LinkedHashMap::new();
            args = Vec::new();
        }
        for (k, v) in &self.key.kwargs {
            kwargs.insert(k.clone(), v.clone());
        }
        let mut dependencies = match self.render_dependencies && any_dependencies {
            true => Some(AST::Subscript(Subscript::new_wrapped(
                params.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped(
                    "dependencies".to_string(),
                    false,
                )),
                false,
            ))),
            false => None,
        };
        let compressed_dependencies = self.key.deps.clone();
        if compressed_dependencies.len() > 0 {
            let left = AST::List(List::new_wrapped(compressed_dependencies, false));
            if let Some(ref right) = dependencies {
                let op = AST::Add(Add::new_wrapped());
                dependencies = Some(AST::BinOp(BinOp::new_wrapped(left, op, right.clone())));
            } else {
                dependencies = Some(left);
            }
        }

        let singleton = T::new(
            self.task_id.clone(),
            new_collector.clone(),
            self.key.get_call(),
            args,
            kwargs,
            dependencies,
            self.key.get_preamble(),
            self.key.get_dialect(),
            endpoints.clone(),
        );
        let statements = singleton.get_statements();
        let items_call = match has_params_dict {
            true => AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    self.params_dict_name.clone(),
                    "items".to_string(),
                    false,
                )),
                Vec::new(),
                LinkedHashMap::new(),
            )),
            false => self.params_dict_name.clone(),
        };
        let for_loop = AST::ForLoop(ForLoop::new_wrapped(
            tpl.clone(),
            items_call,
            statements.clone(),
        ));
        (
            vec![dict_assign, for_loop],
            // TODO: propagate erorr type here
            singleton.get_preamble().unwrap(),
            singleton.get_imports(),
        )
    }
}
