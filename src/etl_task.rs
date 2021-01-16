use crate::constraint::{
    AoristStatement, ArgType, Attribute, Call, ParameterTuple, ParameterTupleDedupKey,
    SimpleIdentifier,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandaloneETLTask {
    /// where the task creation call should be stored.
    task_val: ArgType,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: Option<String>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dep_list: Option<ArgType>,
    /// Python preamble used by this task call
    preamble: Option<String>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
}
/// tuple of:
/// - name of dict / list in which task_val is stored (must be dict or list)
/// - function call (if any)
/// - from parameters:
///   - number of args
///   - names of kwargs
/// - preamble
/// - dialect
pub type ETLTaskCompressionKey = (
    // dict name
    ArgType,
    // function call
    Option<String>,
    // dedup key from parameters
    Option<ParameterTupleDedupKey>,
    // preamble
    Option<String>,
    // dialect
    Option<Dialect>,
);
pub type ETLTaskUncompressiblePart = (
    // dict value
    ArgType,
    // params
    Option<ParameterTuple>,
    // dep list
    Option<ArgType>,
);

impl StandaloneETLTask {
    /// only return true for compressible tasks, i.e. those that have a
    /// dict task val (in the future more stuff could be added here)
    pub fn is_compressible(&self) -> bool {
        match &self.task_val {
            ArgType::Subscript(_) => true,
            _ => false,
        }
    }
    fn get_left_of_task_val(&self) -> Result<ArgType, String> {
        match &self.task_val {
            ArgType::Subscript(x) => {
                let rw = x.read().unwrap();
                Ok(rw.a().clone())
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    pub fn get_compression_key(&self) -> Result<ETLTaskCompressionKey, String> {
        Ok((
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
    pub fn get_uncompressible_part(&self) -> Result<ETLTaskUncompressiblePart, String> {
        Ok((
            self.get_left_of_task_val()?,
            self.params.clone(),
            self.dep_list.clone(),
        ))
    }
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> ArgType {
        self.task_val.clone()
    }
    pub fn new(
        task_val: ArgType,
        call: Option<String>,
        params: Option<ParameterTuple>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_val,
            call,
            params,
            dep_list,
            preamble,
            dialect,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ForLoopETLTask {
    key: ETLTaskCompressionKey,
    values: Vec<ETLTaskUncompressiblePart>,
}
impl ForLoopETLTask {
    pub fn new(key: ETLTaskCompressionKey, values: Vec<ETLTaskUncompressiblePart>) -> Self {
        Self { key, values }
    }
}

pub enum ETLTask {
    StandaloneETLTask(StandaloneETLTask),
    ForLoopETLTask(ForLoopETLTask),
}
