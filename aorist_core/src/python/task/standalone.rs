use crate::flow::{
    CompressibleTask, CompressionKey, ETLFlow, StandaloneTask, TaskBase, UncompressiblePart,
};
use crate::parameter_tuple::ParameterTuple;
use crate::python::task::key::PythonBasedTaskCompressionKey;
use crate::python::task::uncompressible::PythonBasedTaskUncompressiblePart;
use crate::python::{List, PythonImport, PythonPreamble, StringLiteral, AST};
use abi_stable::std_types::ROption;
use aorist_primitives::AoristUniverse;
use aorist_primitives::Dialect;
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandalonePythonBasedTask<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
    /// where the task creation call should be stored.
    task_val: AST,
    /// unique task identifier
    task_id: AString,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: AOption<AString>,
    /// arguments passed to function call
    params: AOption<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dependencies: AVec<AST>,
    /// Python preamble used by this task call
    preamble: AOption<AString>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: AOption<Dialect>,
    singleton_type: PhantomData<T>,
    _universe: PhantomData<U>,
}
impl<T, U> TaskBase<T, U> for StandalonePythonBasedTask<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
}

impl<T, U> StandaloneTask<T, U> for StandalonePythonBasedTask<T, U>
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
    ) -> Self {
        Self {
            task_id,
            task_val,
            call,
            params,
            dependencies,
            preamble,
            dialect,
            singleton_type: PhantomData,
            _universe: PhantomData,
        }
    }
}
impl<T, U> CompressibleTask for StandalonePythonBasedTask<T, U>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
{
    type KeyType = PythonBasedTaskCompressionKey;
    /// only return true for compressible tasks, i.e. those that have a
    /// dict task val (in the future more stuff could be added here)
    fn is_compressible(&self) -> bool {
        match &self.task_val {
            AST::Subscript(_) => true,
            _ => false,
        }
    }
    fn get_compression_key(&self) -> Result<PythonBasedTaskCompressionKey, AString> {
        Ok(PythonBasedTaskCompressionKey::new(
            self.get_left_of_task_val()?,
            self.call.clone(),
            match &self.params {
                AOption(ROption::RSome(p)) => AOption(ROption::RSome(p.get_dedup_key())),
                AOption(ROption::RNone) => AOption(ROption::RNone),
            },
            self.preamble.clone(),
            self.dialect.clone(),
        ))
    }
    fn get_left_of_task_val(&self) -> Result<AST, AString> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read();
                Ok(rw.a().clone())
            }
            _ => Err("Task val must be a subscript".into()),
        }
    }
    fn get_right_of_task_val(&self) -> Result<AString, AString> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read();
                match &rw.b() {
                    AST::StringLiteral(l) => Ok(l.read().value().clone()),
                    _ => Err("Right of subscript must be a string literal".into()),
                }
            }
            _ => Err("Task val must be a subscript".into()),
        }
    }
    fn get_preamble(&self) -> AOption<AString> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}

impl<T, U> StandalonePythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    pub fn get_uncompressible_part(
        &self,
    ) -> Result<PythonBasedTaskUncompressiblePart<T, U>, AString> {
        Ok(PythonBasedTaskUncompressiblePart::new(
            self.task_id.clone(),
            self.get_right_of_task_val()?,
            self.params.clone(),
            self.dependencies.clone(),
        ))
    }
    pub fn get_statements(
        &self,
        endpoints: U::TEndpoints,
    ) -> (AVec<AST>, AVec<PythonPreamble>, AVec<PythonImport>) {
        let args;
        let kwargs;
        if let AOption(ROption::RSome(ref p)) = self.params {
            args = p.get_args();
            kwargs = p.get_kwargs();
        } else {
            args = AVec::new();
            kwargs = LinkedHashMap::new();
        }
        let singleton = T::new(
            AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            self.get_task_val(),
            self.call.clone(),
            args,
            kwargs,
            match self.dependencies.len() {
                0 => AOption(ROption::RNone),
                _ => AOption(ROption::RSome(AST::List(List::new_wrapped(
                    self.dependencies.clone(),
                    false,
                )))),
            },
            self.get_preamble(),
            self.get_dialect(),
            endpoints.clone(),
        );
        (
            singleton.get_statements(),
            // TODO: propagate erorr type here
            singleton.get_preamble().unwrap(),
            singleton.get_imports(),
        )
    }
}
