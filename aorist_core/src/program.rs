use crate::constraint::{OuterConstraint, TConstraint};
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_primitives::{Ancestry, Context, AString};
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

pub trait TProgram<'a, T: TConstraint<'a>> {
    fn new(
        code: AString,
        entrypoint: AString,
        arg_functions: Vec<(Vec<AString>, AString)>,
        kwarg_functions: LinkedHashMap<AString, (Vec<AString>, AString)>,
        dialect: Dialect,
    ) -> Self;
    fn get_arg_functions(&self) -> Vec<(Vec<AString>, AString)>;
    fn get_code(&self) -> AString;
    fn get_dialect(&self) -> Dialect;
    fn get_entrypoint(&self) -> AString;
    fn get_kwarg_functions(&self) -> LinkedHashMap<AString, (Vec<AString>, AString)>;
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
#[derive(Clone)]
pub struct Program<'a, C: OuterConstraint<'a>> {
    _lta: PhantomData<&'a ()>,
    _ltc: PhantomData<C>,
    code: AString,
    arg_functions: Vec<AString>,
    kwarg_functions: LinkedHashMap<AString, AString>,
}
impl<'a, C> Program<'a, C>
where
    C: OuterConstraint<'a>,
{
    pub fn new(
        code: AString,
        arg_functions: Vec<AString>,
        kwarg_functions: LinkedHashMap<AString, AString>,
    ) -> Self {
        Self {
            _lta: PhantomData,
            _ltc: PhantomData,
            code,
            arg_functions,
            kwarg_functions,
        }
    }
}
