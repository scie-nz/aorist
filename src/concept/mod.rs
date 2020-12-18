use crate::constraint::Constraint;
use std::rc::Rc;
use uuid::Uuid;

pub trait AoristConcept {
    fn get_constraints(&self) -> Vec<Rc<Constraint>>;
    fn traverse_constrainable_children(&self);
    fn get_uuid(&self) -> Uuid;
}
