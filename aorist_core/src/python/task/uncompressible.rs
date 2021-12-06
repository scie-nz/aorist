use crate::flow::{ETLFlow, UncompressiblePart};
use crate::parameter_tuple::ParameterTuple;
use aorist_ast::{Dict, List, StringLiteral, AST};
use aorist_primitives::{AString, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PythonBasedTaskUncompressiblePart<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
    // unique task_id
    pub task_id: AString,
    // dict value
    pub dict: AString,
    // params
    pub params: Option<ParameterTuple>,
    // dep list
    pub deps: Vec<AST>,
    singleton_type: PhantomData<T>,
    _universe: PhantomData<U>,
}
impl<T, U> UncompressiblePart<T, U> for PythonBasedTaskUncompressiblePart<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
    fn new(
        task_id: AString,
        dict: AString,
        params: Option<ParameterTuple>,
        deps: Vec<AST>,
    ) -> Self {
        Self {
            task_id,
            dict,
            params,
            deps,
            singleton_type: PhantomData,
            _universe: PhantomData,
        }
    }
    fn as_dict(
        &self,
        insert_deps: bool,
        dependencies_as_list: bool,
        insert_task_name: bool,
    ) -> AST {
        let mut local_params_map: LinkedHashMap<AString, AST> = LinkedHashMap::new();
        if insert_deps {
            let dependencies = match dependencies_as_list {
                true => AST::List(List::new_wrapped(self.deps.clone(), false)),
                false => {
                    assert_eq!(self.deps.len(), 1);
                    self.deps.get(0).unwrap().clone()
                }
            };
            local_params_map.insert("dependencies".into(), dependencies);
        }
        // TODO: get_type should return an enum
        if insert_task_name && T::get_type().as_str() == "airflow" {
            local_params_map.insert(
                "task_id".into(),
                AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            );
        }
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}
