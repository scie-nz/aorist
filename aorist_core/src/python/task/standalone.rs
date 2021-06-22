use crate::dialect::Dialect;
use crate::endpoints::EndpointConfig;
use crate::flow::{
    CompressibleTask, CompressionKey, ETLFlow, StandaloneTask, TaskBase, UncompressiblePart,
};
use crate::parameter_tuple::ParameterTuple;
use crate::python::task::key::PythonBasedTaskCompressionKey;
use crate::python::task::uncompressible::PythonBasedTaskUncompressiblePart;
use crate::python::{List, PythonImport, PythonPreamble, StringLiteral, AST};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use crate::concept::AoristRef;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandalonePythonBasedTask<T>
where
    T: ETLFlow,
{
    /// where the task creation call should be stored.
    task_val: AST,
    /// unique task identifier
    task_id: String,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: Option<String>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dependencies: Vec<AST>,
    /// Python preamble used by this task call
    preamble: Option<String>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
    singleton_type: PhantomData<T>,
}
impl<T> TaskBase<T> for StandalonePythonBasedTask<T> where T: ETLFlow {}

impl<T> StandaloneTask<T> for StandalonePythonBasedTask<T>
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
        }
    }
}
impl<T> CompressibleTask for StandalonePythonBasedTask<T>
where
    T: ETLFlow,
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
    fn get_compression_key(&self) -> Result<PythonBasedTaskCompressionKey, String> {
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
    fn get_left_of_task_val(&self) -> Result<AST, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                Ok(rw.a().clone())
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    fn get_right_of_task_val(&self) -> Result<String, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                match &rw.b() {
                    AST::StringLiteral(l) => Ok(l.read().unwrap().value().clone()),
                    _ => Err("Right of subscript must be a string
                    literal"
                        .to_string()),
                }
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}

impl<T> StandalonePythonBasedTask<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    pub fn get_uncompressible_part(&self) -> Result<PythonBasedTaskUncompressiblePart<T>, String> {
        Ok(PythonBasedTaskUncompressiblePart::new(
            self.task_id.clone(),
            self.get_right_of_task_val()?,
            self.params.clone(),
            self.dependencies.clone(),
        ))
    }
    pub fn get_statements(
        &self,
        endpoints: AoristRef<EndpointConfig>,
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
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}
