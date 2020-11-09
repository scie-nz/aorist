use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::python::TObjectWithPythonCodeGen;
use crate::locations::{GCSLocation, RemoteWebsiteLocation, HiveLocation};

#[enum_dispatch(RemoteWebsiteLocation, HiveLocation)]
pub trait TObjectWithPrefectCodeGen: TObjectWithPythonCodeGen {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>);
}
