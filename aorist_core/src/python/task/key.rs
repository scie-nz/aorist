use crate::flow::CompressionKey;
use crate::parameter_tuple::ParameterTupleDedupKey;
use crate::python::AST;
use aorist_primitives::Dialect;
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;

/// tuple of:
/// - name of dict / list in which task_val is stored (must be dict or list)
/// - function call (if any)
/// - from parameters:
///   - number of args
///   - names of kwargs
/// - preamble
/// - dialect
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PythonBasedTaskCompressionKey {
    // dict name
    dict_name: AST,
    // function call
    function_call: AOption<AString>,
    // dedup key from parameters
    dedup_key: AOption<ParameterTupleDedupKey>,
    // preamble
    preamble: AOption<AString>,
    // dialect
    dialect: AOption<Dialect>,
    // optional: dependencies
    pub deps: AVec<AST>,
    // optional: kwargs
    pub kwargs: LinkedHashMap<AString, AST>,
}
impl CompressionKey for PythonBasedTaskCompressionKey {
    fn new(
        dict_name: AST,
        function_call: AOption<AString>,
        dedup_key: AOption<ParameterTupleDedupKey>,
        preamble: AOption<AString>,
        dialect: AOption<Dialect>,
    ) -> Self {
        Self {
            dict_name,
            function_call,
            dedup_key,
            preamble,
            dialect,
            deps: AVec::new(),
            kwargs: LinkedHashMap::new(),
        }
    }
    fn get_dict_name(&self) -> AST {
        self.dict_name.clone()
    }
    fn get_dedup_key(&self) -> AOption<ParameterTupleDedupKey> {
        self.dedup_key.clone()
    }
    fn get_call(&self) -> AOption<AString> {
        self.function_call.clone()
    }
    fn get_preamble(&self) -> AOption<AString> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
}
