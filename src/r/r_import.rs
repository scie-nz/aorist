use crate::code::Import;
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
}
