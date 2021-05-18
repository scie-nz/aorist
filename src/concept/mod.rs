use std::sync::{Arc, RwLock};
use uuid::Uuid;
pub use aorist_core::AoristConcept;

pub trait AoristConceptChildren {
    fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>>
    where
        'a: 'b;
}
pub trait Ancestry {}
include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
