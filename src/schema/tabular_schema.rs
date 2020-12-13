#![allow(non_snake_case)]
use crate::template::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TabularSchema {
    datumTemplateName: String,
    attributes: Vec<String>,
}
impl TabularSchema {
    pub fn get_presto_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        assert!(templates.contains_key(&self.datumTemplateName));
        let template = templates.get(&self.datumTemplateName).unwrap();
        let columnSchema = template.get_presto_schema(&self.attributes);
        format!("{}", columnSchema)
    }
    pub fn get_orc_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        assert!(templates.contains_key(&self.datumTemplateName));
        let template = templates.get(&self.datumTemplateName).unwrap();
        let columnSchema = template.get_orc_schema(&self.attributes);
        format!("{}", columnSchema)
    }
}
