mod ast;
mod program;

use crate::constraint::{AoristStatement, Import};
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;
pub type PythonStatementInput = (
    Vec<AoristStatement>,
    LinkedHashSet<String>,
    BTreeSet<Import>,
);

pub use program::PythonProgram;
