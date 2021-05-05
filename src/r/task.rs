use crate::dialect::Dialect;
use crate::flow::{ETLFlow, ETLTask, StandaloneTask, TaskBase};
use crate::parameter_tuple::ParameterTuple;
use crate::python::AST;
use crate::r::r_import::RImport;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandaloneRBasedTask<T>
where
    T: ETLFlow,
{
    /// unique task identifier
    task_id: String,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: Option<String>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// R preamble used by this task call
    preamble: Option<String>,
    /// Dialect (e.g. Bash, R, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
    singleton_type: PhantomData<T>,
}

impl<T> StandaloneTask<T> for StandaloneRBasedTask<T>
where
    T: ETLFlow,
{
    fn new(
        task_id: String,
        _task_val: AST,
        call: Option<String>,
        params: Option<ParameterTuple>,
        _dependencies: Vec<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_id,
            call,
            params,
            preamble,
            dialect,
            singleton_type: PhantomData,
        }
    }
}
impl<T> TaskBase<T> for StandaloneRBasedTask<T>
where
    T: ETLFlow {
    type I = RImport;
}
pub enum RBasedTask<T>
where
    T: ETLFlow,
{
    StandaloneRBasedTask(StandaloneRBasedTask<T>),
}
impl<T> ETLTask<T> for RBasedTask<T>
where
    T: ETLFlow,
{
    type S = StandaloneRBasedTask<T>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandaloneRBasedTask(task)
    }
}
