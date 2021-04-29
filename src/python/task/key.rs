use crate::dialect::Dialect;
use crate::flow::CompressionKey;
use crate::parameter_tuple::ParameterTupleDedupKey;
use crate::python::AST;
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
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PythonBasedTaskCompressionKey {
    // dict name
    dict_name: AST,
    // function call
    function_call: Option<String>,
    // dedup key from parameters
    dedup_key: Option<ParameterTupleDedupKey>,
    // preamble
    preamble: Option<String>,
    // dialect
    dialect: Option<Dialect>,
    // optional: dependencies
    pub deps: Vec<AST>,
    // optional: kwargs
    pub kwargs: LinkedHashMap<String, AST>,
}
impl CompressionKey for PythonBasedTaskCompressionKey {
    fn new(
        dict_name: AST,
        function_call: Option<String>,
        dedup_key: Option<ParameterTupleDedupKey>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            dict_name,
            function_call,
            dedup_key,
            preamble,
            dialect,
            deps: Vec::new(),
            kwargs: LinkedHashMap::new(),
        }
    }
    fn get_dict_name(&self) -> AST {
        self.dict_name.clone()
    }
    fn get_dedup_key(&self) -> Option<ParameterTupleDedupKey> {
        self.dedup_key.clone()
    }
    fn get_call(&self) -> Option<String> {
        self.function_call.clone()
    }
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
}
