#![allow(non_snake_case)]
use crate::access_policy::*;
use crate::asset::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::template::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct DataSet {
    pub name: String,
    #[constrainable]
    #[py_default = "Vec::new()"]
    accessPolicies: Vec<AccessPolicy>,
    #[constrainable]
    pub datumTemplates: Vec<DatumTemplate>,
    #[constrainable]
    assets: Vec<Asset>,
}
impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub trait TDataSet {
    fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate>;
}
impl TDataSet for DataSet {
    fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
impl InnerDataSet {
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, InnerDatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
#[pymethods]
impl InnerDataSet {
    pub fn add_template(&mut self, t: InnerDatumTemplate) -> PyResult<()> {
        self.datumTemplates.push(t);
        Ok(())
    }
    pub fn add_asset(&mut self, a: InnerAsset) -> PyResult<()> {
        self.assets.push(a);
        Ok(())
    }
}
