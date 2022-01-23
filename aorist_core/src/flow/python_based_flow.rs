use crate::flow::etl_flow::ETLFlow;
use crate::python::{NativePythonPreamble, PythonPreamble, RPythonPreamble};
use abi_stable::std_types::ROption;
use aorist_primitives::AoristUniverse;
use aorist_primitives::Dialect;
use aorist_primitives::TPrestoEndpoints;
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use pyo3::prelude::*;

pub trait PythonBasedFlow<U>: ETLFlow<U>
where
    U: AoristUniverse,
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> AOption<AString>;
    fn get_python_preamble(&self) -> PyResult<AVec<PythonPreamble>> {
        let preambles = match self.get_dialect() {
            AOption(ROption::RSome(Dialect::Python(_))) => match self.get_preamble_string() {
                AOption(ROption::RSome(p)) => Ok(vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p)?,
                )]
                .into_iter()
                .collect()),
                AOption(ROption::RNone) => Ok(AVec::new()),
            },
            AOption(ROption::RSome(Dialect::R(_))) => match self.get_preamble_string() {
                AOption(ROption::RSome(p)) => Ok(vec![PythonPreamble::RPythonPreamble(
                    RPythonPreamble::new(p)?,
                )]
                .into_iter()
                .collect()),
                AOption(ROption::RNone) => Ok(AVec::new()),
            },
            _ => Ok(AVec::new()),
        };
        preambles
    }
}
