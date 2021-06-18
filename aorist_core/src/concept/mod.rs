pub use aorist_primitives::{ConceptEnum, AoristConcept};

pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a>;
}
