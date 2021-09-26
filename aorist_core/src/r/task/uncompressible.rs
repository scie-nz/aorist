use crate::flow::{ETLFlow, UncompressiblePart};
use crate::parameter_tuple::ParameterTuple;
use aorist_ast::{Dict, AST};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RBasedTaskUncompressiblePart<T>
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
impl<T> UncompressiblePart<T> for RBasedTaskUncompressiblePart<T>
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
    fn as_dict(&self, _insert_deps: bool, _dependencies_as_list: bool, insert_task_name: bool) -> AST {
        let mut local_params_map: LinkedHashMap<String, AST> = LinkedHashMap::new();
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}
