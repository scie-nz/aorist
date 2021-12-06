use crate::code::Import;
use aorist_ast::{Call, SimpleIdentifier, StringLiteral, AST};
use extendr_api::prelude::*;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RImport {
    pub library: AString,
}
impl Import for RImport {}
impl RImport {
    pub fn new(library: AString) -> Self {
        Self { library }
    }
    pub fn to_r_ast_node(&self, depth: usize) -> Robj {
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("library".into())),
            vec![AST::StringLiteral(StringLiteral::new_wrapped(
                self.library.clone(),
                false,
            ))],
            linked_hash_map::LinkedHashMap::new(),
        ))
        .to_r_ast_node(depth)
    }
}
