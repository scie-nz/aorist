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
    pub assets: Vec<Asset>,
}
impl DataSet {
    pub fn get_template_for_asset<T: TAsset>(&self, asset: &T) -> Result<DatumTemplate, String> {
        let schema = asset.get_schema();
        let template_name = schema.get_datum_template_name();
        let mapped_templates = self.get_mapped_datum_templates();
        let template = mapped_templates.get(&template_name);
        match template {
            Some(template) => Ok(template.clone()),
            None => Err(format!(
                "Could not find template for asset {} in dataset {}",
                asset.get_name(),
                self.name,
            )),
        }
    }
    pub fn get_asset(&self, name: String) -> Result<Asset, String> {
        for asset in self.assets.iter() {
            if asset.get_name() == name {
                return Ok(asset.clone());
            }
        }
        Err(format!("Could not find asset {} in dataset {}.", name, self.name).to_string())
    }
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
    pub fn get_name(&self) -> &String {
        &self.name
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
