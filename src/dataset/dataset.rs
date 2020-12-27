#![allow(non_snake_case)]
use crate::access_policy::AccessPolicy;
use crate::asset::Asset;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::object::TAoristObject;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Default, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct DataSet {
    name: String,
    #[constrainable]
    accessPolicies: Vec<AccessPolicy>,
    #[constrainable]
    datumTemplates: Vec<DatumTemplate>,
    #[constrainable]
    assets: Vec<Asset>,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl DataSet {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
}
