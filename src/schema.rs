#![allow(non_snake_case)]
use crate::templates::DatumTemplate;
use serde::{Serialize, Deserialize};
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
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum DataSchema {
    TabularSchema(TabularSchema),
}
impl DataSchema {
    pub fn get_presto_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        match self {
            DataSchema::TabularSchema(x) => x.get_presto_schema(templates),
        }
    }
}
