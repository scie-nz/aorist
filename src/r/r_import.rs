use crate::code::Import;
use extendr_api::prelude::*;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RImport {
    library: String,
}
impl Import for RImport {}
impl RImport {
    pub fn new(library: String) -> Self {
        Self { library }
    }
    pub fn to_r_ast_node(&self, _depth: usize) -> Robj {
        r!(Lang(&[r!(Symbol("library")), r!(&self.library)]))
    }
}
