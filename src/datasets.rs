#![allow(non_snake_case)]
use crate::access_policies::AccessPolicy;
use crate::assets::Asset;
use crate::templates::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::object::TAoristObject;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSet {
    name: String,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
    assets: Vec<Asset>,
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
    pub fn get_presto_schemas(&self) -> String {
        let mappedTemplates = self.get_mapped_datum_templates();
        let mut schemas: String = "".to_string();
        for asset in &self.assets {
            let schema = asset.get_presto_schemas(&mappedTemplates);
            schemas += "\n\n";
            schemas += &schema;
        }
        schemas
    }
}
