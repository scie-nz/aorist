use crate::constraint::{OuterConstraint, TConstraint};
use aorist_primitives::Dialect;
use crate::parameter_tuple::ParameterTuple;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_primitives::{AString, AVec, Ancestry, Context};
use linked_hash_map::LinkedHashMap;

pub trait TProgram<'a, T: TConstraint<'a>> {
    fn new(
        code: AString,
        entrypoint: AString,
        arg_functions: AVec<(AVec<AString>, AString)>,
        kwarg_functions: LinkedHashMap<AString, (AVec<AString>, AString)>,
        dialect: Dialect,
    ) -> Self;
    fn get_arg_functions(&self) -> AVec<(AVec<AString>, AString)>;
    fn get_code(&self) -> AString;
    fn get_dialect(&self) -> Dialect;
    fn get_entrypoint(&self) -> AString;
    fn get_kwarg_functions(&self) -> LinkedHashMap<AString, (AVec<AString>, AString)>;
}
pub trait TOuterProgram: Clone {
    type TAncestry: Ancestry;
    fn get_dialect(&self) -> Dialect;
    fn compute_args<'a, T: OuterConstraint<'a>>(
        &self,
        root: <Self::TAncestry as Ancestry>::TConcept,
        ancestry: &Self::TAncestry,
        context: &mut Context,
        constraint: RArc<RRwLock<T>>,
    ) -> (AString, AString, ParameterTuple, Dialect);
}
