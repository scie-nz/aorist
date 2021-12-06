use crate::dialect::Dialect;
use crate::flow::{
    CompressibleTask, CompressionKey, ETLFlow, StandaloneTask, TaskBase, UncompressiblePart,
};
use crate::parameter_tuple::ParameterTuple;
use crate::python::task::key::PythonBasedTaskCompressionKey;
use crate::python::task::uncompressible::PythonBasedTaskUncompressiblePart;
use crate::python::{List, PythonImport, PythonPreamble, StringLiteral, AST};
use aorist_primitives::{AString, AoristUniverse};
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
    call: Option<AString>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dependencies: Vec<AST>,
    /// Python preamble used by this task call
    preamble: Option<AString>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
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
        call: Option<AString>,
        params: Option<ParameterTuple>,
        dependencies: Vec<AST>,
        preamble: Option<AString>,
        dialect: Option<Dialect>,
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
                Some(p) => Some(p.get_dedup_key()),
                None => None,
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
    fn get_preamble(&self) -> Option<AString> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
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
    ) -> (Vec<AST>, Vec<PythonPreamble>, Vec<PythonImport>) {
        let args;
        let kwargs;
        if let Some(ref p) = self.params {
            args = p.get_args();
            kwargs = p.get_kwargs();
        } else {
            args = Vec::new();
            kwargs = LinkedHashMap::new();
        }
        let singleton = T::new(
            AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone(), false)),
            self.get_task_val(),
            self.call.clone(),
            args,
            kwargs,
            match self.dependencies.len() {
                0 => None,
                _ => Some(AST::List(List::new_wrapped(
                    self.dependencies.clone(),
                    false,
                ))),
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
