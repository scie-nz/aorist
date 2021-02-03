#![allow(non_snake_case)]
use crate::access_policy::AccessPolicy;
use crate::asset::Asset;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::object::TAoristObject;
use crate::template::{DatumTemplate, TDatumTemplate};
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct DataSet {
    name: String,
    #[constrainable]
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
#[pymethods]
impl DataSet {
    #[new]
    #[args(accessPolicies = "Vec::new()")]
    fn new(
        name: String,
        accessPolicies: Vec<AccessPolicy>,
        datumTemplates: Vec<DatumTemplate>,
        assets: Vec<Asset>,
    ) -> Self {
        Self {
            name,
            accessPolicies,
            datumTemplates,
            assets,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
impl TDataSet for DataSet {
    fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
