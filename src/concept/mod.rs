use crate::constraint::Constraint;

pub trait AoristConcept {
    fn get_constraints(&self) -> Vec<Constraint>;
    fn traverse_constrainable_children(&self);
}
