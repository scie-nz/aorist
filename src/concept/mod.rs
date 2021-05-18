use std::sync::{Arc, RwLock};
use uuid::Uuid;
pub use aorist_core::AoristConcept;

pub trait ConceptEnum<'a> {}
pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a>;
}

pub trait AoristConceptChildren {
    fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>>
    where
        'a: 'b;
}

include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
