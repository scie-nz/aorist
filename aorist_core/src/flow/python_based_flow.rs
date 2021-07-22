use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::python::{NativePythonPreamble, PythonPreamble, RPythonPreamble};
use aorist_primitives::AoristUniverse;
use aorist_primitives::TPrestoEndpoints;

pub trait PythonBasedFlow<U>: ETLFlow<U>
where
    U: AoristUniverse,
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<String>;
    fn get_python_preamble(&self) -> Vec<PythonPreamble> {
        let preambles = match self.get_dialect() {
            Some(Dialect::Python(_)) => match self.get_preamble_string() {
                Some(p) => vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p),
                )],
                None => Vec::new(),
            },
            Some(Dialect::R(_)) => match self.get_preamble_string() {
                Some(p) => vec![PythonPreamble::RPythonPreamble(RPythonPreamble::new(p))],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }
}
