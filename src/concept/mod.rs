use crate::constraint::Constraint;
use std::rc::Rc;

pub trait AoristConcept {
    fn get_constraints(self) -> Vec<Rc<Constraint>>;
    fn traverse_constrainable_children(&self);
}
