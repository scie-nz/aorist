use crate::code::Import;
use std::hash::Hash;

pub trait Preamble: Eq + Hash + Clone
where
    Self::ImportType: Import,
{
    type ImportType;
    fn get_imports(&self) -> Vec<Self::ImportType>;
}
