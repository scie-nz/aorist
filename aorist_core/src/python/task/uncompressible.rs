use crate::flow::{ETLFlow, UncompressiblePart};
use aorist_ast::{Dict, List, StringLiteral, AST};
use crate::parameter_tuple::ParameterTuple;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PythonBasedTaskUncompressiblePart<T>
where
    T: ETLFlow,
{
    // unique task_id
    pub task_id: String,
    // dict value
    pub dict: String,
    // params
    pub params: Option<ParameterTuple>,
    // dep list
    pub deps: Vec<AST>,
    singleton_type: PhantomData<T>,
}
impl<T> UncompressiblePart<T> for PythonBasedTaskUncompressiblePart<T>
where
    T: ETLFlow,
{
    fn new(task_id: String, dict: String, params: Option<ParameterTuple>, deps: Vec<AST>) -> Self {
        Self {
            task_id,
            dict,
            params,
            deps,
            singleton_type: PhantomData,
        }
    }
    fn as_dict(&self, dependencies_as_list: bool, insert_task_name: bool) -> AST {
        let mut local_params_map: LinkedHashMap<String, AST> = LinkedHashMap::new();
        if self.deps.len() > 0 {
            let dependencies = match dependencies_as_list {
                true => AST::List(List::new_wrapped(self.deps.clone(), false)),
                false => {
                    assert_eq!(self.deps.len(), 1);
                    self.deps.get(0).unwrap().clone()
                }
            };
            local_params_map.insert("dependencies".to_string(), dependencies);
        }
        // TODO: get_type should return an enum
        if insert_task_name && T::get_type() == "airflow".to_string() {
            local_params_map.insert(
                "task_id".to_string(),
                AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            );
        }
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}
