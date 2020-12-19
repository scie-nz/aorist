#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct TabularSchema {
    datumTemplateName: String,
    attributes: Vec<String>,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
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
