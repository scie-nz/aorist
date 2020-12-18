use crate::constraint::Constraint;
use std::rc::Rc;
use uuid::Uuid;

pub trait AoristConcept {
    // TODO: should be renamed to compute_bottom_up_constraints
    fn compute_constraints(&mut self);
    fn get_constraints(&self) -> &Vec<Rc<Constraint>>;
    fn get_downstream_constraints(&self) -> Vec<Rc<Constraint>>;
    // TODO: should be renamed to compute_top_down_constraints
    fn traverse_constrainable_children(&self, upstream_constraints: Vec<Rc<Constraint>>);
    fn get_uuid(&self) -> Uuid;
}
