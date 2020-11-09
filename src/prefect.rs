use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::python::TObjectWithPythonCodeGen;
use crate::locations::{GCSLocation, RemoteWebsiteLocation, HiveLocation};
use crate::assets::{Asset, StaticDataTable};

#[enum_dispatch(RemoteWebsiteLocation, HiveLocation, Storage, StorageSetup, Asset)]
pub trait TObjectWithPrefectCodeGen: TObjectWithPythonCodeGen {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>);
}

#[enum_dispatch(Asset, StorageSetup)]
pub trait TObjectWithPrefectDAGCodeGen: TObjectWithPrefectCodeGen {
    fn get_prefect_dag(&self) -> Result<String, String>;
}
