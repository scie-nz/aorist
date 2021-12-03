use crate::constraint::{OuterConstraint, TConstraint};
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_primitives::{Ancestry, Context};
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

pub trait TProgram<'a, T: TConstraint<'a>> {
    fn new(
        code: String,
        entrypoint: String,
        arg_functions: Vec<(Vec<String>, String)>,
        kwarg_functions: LinkedHashMap<String, (Vec<String>, String)>,
        dialect: Dialect,
    ) -> Self;
    fn get_arg_functions(&self) -> Vec<(Vec<String>, String)>;
    fn get_code(&self) -> String;
    fn get_dialect(&self) -> Dialect;
    fn get_entrypoint(&self) -> String;
    fn get_kwarg_functions(&self) -> LinkedHashMap<String, (Vec<String>, String)>;
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
    ) -> (String, String, ParameterTuple, Dialect);
}
#[derive(Clone)]
pub struct Program<'a, C: OuterConstraint<'a>> {
    _lta: PhantomData<&'a ()>,
    _ltc: PhantomData<C>,
    code: String,
    arg_functions: Vec<String>,
    kwarg_functions: LinkedHashMap<String, String>,
}
impl<'a, C> Program<'a, C>
where
    C: OuterConstraint<'a>,
{
    pub fn new(
        code: String,
        arg_functions: Vec<String>,
        kwarg_functions: LinkedHashMap<String, String>,
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
