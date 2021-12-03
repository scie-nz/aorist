use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;

pub trait TAssignmentTarget
where
    Self: Sized,
{
    fn as_assignment_target(&self) -> Self;
    fn as_wrapped_assignment_target(&self) -> RArc<RRwLock<Self>> {
        RArc::new(RRwLock::new(self.as_assignment_target()))
    }
}
