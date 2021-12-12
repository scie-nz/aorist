use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use aorist_primitives::AVec;
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
    pub task_id: AString,
    // dict value
    pub dict: AString,
    // params
    pub params: AOption<ParameterTuple>,
    // dep list
    pub deps: AVec<AST>,
    singleton_type: PhantomData<T>,
}
impl<T> UncompressiblePart<T> for RBasedTaskUncompressiblePart<T>
where
    T: ETLFlow,
{
    fn new(task_id: AString, dict: AString, params: AOption<ParameterTuple>, deps: AVec<AST>) -> Self {
        Self {
            task_id,
            dict,
            params,
            deps,
            singleton_type: PhantomData,
        }
    }
    fn as_dict(&self, _insert_deps: bool, _dependencies_as_list: bool, insert_task_name: bool) -> AST {
        let mut local_params_map: LinkedHashMap<AString, AST> = LinkedHashMap::new();
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}
