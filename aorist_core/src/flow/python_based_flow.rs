use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::python::{NativePythonPreamble, PythonPreamble, RPythonPreamble};
use aorist_primitives::AoristUniverse;
use aorist_primitives::TPrestoEndpoints;
use pyo3::prelude::*;

pub trait PythonBasedFlow<U>: ETLFlow<U>
where
    U: AoristUniverse,
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<String>;
    fn get_python_preamble(&self) -> PyResult<Vec<PythonPreamble>> {
        let preambles = match self.get_dialect() {
            Some(Dialect::Python(_)) => match self.get_preamble_string() {
                Some(p) => Ok(vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p)?,
                )]),
                None => Ok(Vec::new()),
            },
            Some(Dialect::R(_)) => match self.get_preamble_string() {
                Some(p) => Ok(vec![PythonPreamble::RPythonPreamble(RPythonPreamble::new(
                    p,
                )?)]),
                None => Ok(Vec::new()),
            },
            _ => Ok(Vec::new()),
        };
        preambles
    }
}
