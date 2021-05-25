pub use aorist_core::{Ancestry, AoristConcept, ConceptEnum};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct WrappedConcept<'a, T> {
    pub inner: T,
    pub _phantom_lt: std::marker::PhantomData<&'a ()>,
}

pub trait AoristConceptChildren {
    fn get_child_concepts<'a, 'b>(&'a self) -> Vec<Concept<'b>>
    where
        'a: 'b;
}

include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
