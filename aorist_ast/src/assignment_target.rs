use abi_stable::std_types::RArc;
use std::sync::RwLock;

pub trait TAssignmentTarget
where
    Self: Sized,
{
    fn as_assignment_target(&self) -> Self;
    fn as_wrapped_assignment_target(&self) -> RArc<RwLock<Self>> {
        RArc::new(RwLock::new(self.as_assignment_target()))
    }
}
