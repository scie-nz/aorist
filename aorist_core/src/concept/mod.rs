pub use aorist_primitives::{AoristConcept, ConceptEnum};

pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a>;
}
