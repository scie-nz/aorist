use aorist_primitives::Ancestry;
use crate::dialect::Dialect;
use crate::concept::{ConceptAncestry, Concept, AoristRef};
use crate::constraint::{OuterConstraint, TConstraint};
use std::marker::PhantomData;
use linked_hash_map::LinkedHashMap;  
use crate::parameter_tuple::ParameterTuple;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyString; 
#[cfg(feature = "python")]
use pyo3::pycell::PyCell;

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
    fn compute_args(
        &self,
        root: <Self::TAncestry as Ancestry>::TConcept,
        ancestry: &Self::TAncestry,
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
    pub fn new(code: String, arg_functions: Vec<String>, kwarg_functions: LinkedHashMap<String, String>) -> Self {
        Self {
            _lta: PhantomData,
            _ltc: PhantomData,
            code,
            arg_functions,
            kwarg_functions,
        }
    }

    /*
    #[cfg(feature = "python")]
    pub fn compute_arg_lambdas(&self, obj: ) -> Vec<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let mut result = Vec::new();
        let dill: &PyModule = PyModule::import(py, "dill").unwrap();
        for serialized in &self.arg_functions {
            let py_arg = PyString::new(py, &serialized);
            let deserialized = dill.call1("loads", (py_arg,)).unwrap();
            result.push(deserialized.into_ref(py));
        }
        result
    }*/
}
