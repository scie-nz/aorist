use crate::dialect::Dialect;
use crate::flow::CompressionKey;
use crate::parameter_tuple::ParameterTupleDedupKey;
use aorist_ast::AST;
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
pub struct RBasedTaskCompressionKey {
    // dict name
    dict_name: AST,
    // function call
    function_call: Option<AString>,
    // dedup key from parameters
    dedup_key: Option<ParameterTupleDedupKey>,
    // preamble
    preamble: Option<AString>,
    // dialect
    dialect: Option<Dialect>,
    // optional: dependencies
    pub deps: Vec<AST>,
    // optional: kwargs
    pub kwargs: LinkedHashMap<AString, AST>,
}
impl CompressionKey for RBasedTaskCompressionKey {
    fn new(
        dict_name: AST,
        function_call: Option<AString>,
        dedup_key: Option<ParameterTupleDedupKey>,
        preamble: Option<AString>,
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
    fn get_call(&self) -> Option<AString> {
        self.function_call.clone()
    }
    fn get_preamble(&self) -> Option<AString> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
}
