use crate::code::Import;
use crate::python::{Call, SimpleIdentifier, StringLiteral, AST};
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
    pub fn to_r_ast_node(&self, depth: usize) -> Robj {
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("library".to_string())),
            vec![AST::StringLiteral(StringLiteral::new_wrapped(
                self.library.clone(),
                false,
            ))],
            linked_hash_map::LinkedHashMap::new(),
        ))
        .to_r_ast_node(depth)
    }
}
