use aorist_primitives::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::parameter_tuple::{ParameterTuple, ParameterTupleDedupKey};
use aorist_ast::AST;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristUniverse};
use std::hash::Hash;

pub trait TaskBase<T, U: AoristUniverse>
where
    T: ETLFlow<U>,
{
}

pub trait StandaloneTask<T, U>: TaskBase<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
    fn new(
        task_id: AString,
        task_val: AST,
        call: AOption<AString>,
        params: AOption<ParameterTuple>,
        dependencies: AVec<AST>,
        preamble: AOption<AString>,
        dialect: AOption<Dialect>,
    ) -> Self;
}
pub trait CompressionKey: Clone + Hash + PartialEq + Eq {
    fn new(
        dict_name: AST,
        function_call: AOption<AString>,
        dedup_key: AOption<ParameterTupleDedupKey>,
        preamble: AOption<AString>,
        dialect: AOption<Dialect>,
    ) -> Self;
    fn get_dict_name(&self) -> AST;
    fn get_dedup_key(&self) -> AOption<ParameterTupleDedupKey>;
    fn get_call(&self) -> AOption<AString>;
    fn get_preamble(&self) -> AOption<AString>;
    fn get_dialect(&self) -> AOption<Dialect>;
}

pub trait CompressibleTask
where
    Self::KeyType: CompressionKey,
{
    type KeyType;
    fn is_compressible(&self) -> bool;
    fn get_compression_key(&self) -> Result<Self::KeyType, AString>;
    fn get_left_of_task_val(&self) -> Result<AST, AString>;
    fn get_right_of_task_val(&self) -> Result<AString, AString>;
    fn get_preamble(&self) -> AOption<AString>;
    fn get_dialect(&self) -> AOption<Dialect>;
    fn get_task_val(&self) -> AST;
}
pub trait ETLTask<T, U: AoristUniverse>: TaskBase<T, U>
where
    T: ETLFlow<U>,
    Self::S: StandaloneTask<T, U>,
    Self::S: TaskBase<T, U>,
{
    type S;

    fn standalone_task(task: Self::S) -> Self;
}
pub trait CompressibleETLTask<T, U: AoristUniverse>
where
    T: ETLFlow<U>,
    Self: ETLTask<T, U>,
    Self::S: CompressibleTask,
{
    type F;
}
pub trait UncompressiblePart<T, U: AoristUniverse>
where
    T: ETLFlow<U>,
{
    fn new(
        task_id: AString,
        dict: AString,
        params: AOption<ParameterTuple>,
        deps: AVec<AST>,
    ) -> Self;
    fn as_dict(&self, insert_deps: bool, dependencies_as_list: bool, insert_task_name: bool)
        -> AST;
}
pub trait ForLoopCompressedTask<T, U: AoristUniverse>
where
    T: ETLFlow<U>,
{
    type KeyType: CompressionKey;
    type UncompressiblePartType: UncompressiblePart<T, U>;
    fn new(
        params_dict_name: AST,
        key: Self::KeyType,
        values: AVec<Self::UncompressiblePartType>,
        task_id: AST,
        insert_task_name: bool,
        render_dependencies: bool,
    ) -> Self;
}
