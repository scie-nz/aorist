use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::python::AST;
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
