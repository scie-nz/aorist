use crate::constraint::Constraint;
use std::rc::Rc;
use uuid::Uuid;

pub trait AoristConcept {
    fn compute_constraints(&mut self);
    fn get_constraints(&self) -> &Vec<Rc<Constraint>>;
    fn traverse_constrainable_children(&self, upstream_constraints: Vec<Rc<Constraint>>);
    fn get_uuid(&self) -> Uuid;
}
