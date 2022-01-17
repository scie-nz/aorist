use aorist_primitives::AOption;
use abi_stable::std_types::ROption;

use aorist_primitives::Dialect;
use crate::endpoints::EndpointConfig;
use crate::flow::{
    CompressibleTask, CompressionKey, ETLFlow, StandaloneTask, TaskBase, UncompressiblePart,
};
use crate::parameter_tuple::ParameterTuple;
use crate::r::task::{RBasedTaskCompressionKey, RBasedTaskUncompressiblePart};
use crate::r::{RImport, RPreamble};
use aorist_ast::{List, StringLiteral, AST};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandaloneRBasedTask<T>
where
    T: ETLFlow,
{
    /// unique task identifier
    task_id: AString,
    task_val: AST,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: AOption<AString>,
    /// arguments passed to function call
    params: AOption<ParameterTuple>,
    /// R preamble used by this task call
    preamble: AOption<AString>,
    /// Dialect (e.g. Bash, R, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: AOption<Dialect>,
    dependencies: AVec<AST>,
    singleton_type: PhantomData<T>,
}
impl<T> StandaloneRBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    // TODO: move to trait
    pub fn get_uncompressible_part(&self) -> Result<RBasedTaskUncompressiblePart<T>, AString> {
        Ok(RBasedTaskUncompressiblePart::new(
            self.task_id.clone(),
            self.get_right_of_task_val()?,
            self.params.clone(),
            self.dependencies.clone(),
        ))
    }
    pub fn get_preamble(&self) -> AOption<AString> {
        self.preamble.clone()
    }
    pub fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
    pub fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (AVec<AST>, AVec<RPreamble>, AVec<RImport>) {
        let args;
        let kwargs;
        if let Some(ref p) = self.params {
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
impl<T> StandaloneTask<T> for StandaloneRBasedTask<T>
where
    T: ETLFlow,
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
            preamble,
            dependencies,
            dialect,
            singleton_type: PhantomData,
        }
    }
}
impl<T> TaskBase<T> for StandaloneRBasedTask<T> where T: ETLFlow {}
impl<T> CompressibleTask for StandaloneRBasedTask<T>
where
    T: ETLFlow,
{
    type KeyType = RBasedTaskCompressionKey;
    /// only return true for compressible tasks, i.e. those that have a
    /// dict task val (in the future more stuff could be added here)
    fn is_compressible(&self) -> bool {
        match &self.task_val {
            AST::Subscript(_) => true,
            _ => false,
        }
    }
    fn get_compression_key(&self) -> Result<RBasedTaskCompressionKey, AString> {
        Ok(RBasedTaskCompressionKey::new(
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
                    _ => Err("Right of subscript must be a string
                    literal"
                        .to_string()),
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
