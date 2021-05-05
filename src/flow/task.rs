use crate::code::Import;
use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::parameter_tuple::{ParameterTuple, ParameterTupleDedupKey};
use crate::python::AST;
use std::hash::Hash;

pub trait TaskBase<T>
where
    T: ETLFlow,
{
    type I: Import;
}

pub trait StandaloneTask<T>: TaskBase<T>
where
    T: ETLFlow,
{
    fn new(
        task_id: String,
        task_val: AST,
        call: Option<String>,
        params: Option<ParameterTuple>,
        dependencies: Vec<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self;
}
pub trait CompressionKey: Clone + Hash + PartialEq + Eq {
    fn new(
        dict_name: AST,
        function_call: Option<String>,
        dedup_key: Option<ParameterTupleDedupKey>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self;
    fn get_dict_name(&self) -> AST;
    fn get_dedup_key(&self) -> Option<ParameterTupleDedupKey>;
    fn get_call(&self) -> Option<String>;
    fn get_preamble(&self) -> Option<String>;
    fn get_dialect(&self) -> Option<Dialect>;
}

pub trait CompressibleTask
where
    Self::KeyType: CompressionKey,
{
    type KeyType;
    fn is_compressible(&self) -> bool;
    fn get_compression_key(&self) -> Result<Self::KeyType, String>;
    fn get_left_of_task_val(&self) -> Result<AST, String>;
    fn get_right_of_task_val(&self) -> Result<String, String>;
    fn get_preamble(&self) -> Option<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> AST;

}
pub trait ETLTask<T>: TaskBase<T>
where
    T: ETLFlow,
    Self::S: StandaloneTask<T>,
    Self::S: TaskBase<T, I = <Self as TaskBase<T>>::I>,
{
    type S;

    fn standalone_task(task: Self::S) -> Self;
}
pub trait CompressibleETLTask<T>
where
    T: ETLFlow,
    Self: ETLTask<T>,
    Self::S: CompressibleTask,
{
    type F;
}
pub trait UncompressiblePart<T>
where
    T: ETLFlow {
    fn new(
        task_id: String,
        dict: String,
        params: Option<ParameterTuple>,
        deps: Vec<AST>,
    ) -> Self;
    fn as_dict(&self, dependencies_as_list: bool, insert_task_name: bool) -> AST;
}
