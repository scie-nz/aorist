#![allow(non_snake_case)]
use crate::asset::static_data_table::StaticDataTable;
use crate::concept::AoristConcept;
use crate::constraint::{AoristConstraint, Constraint};
use crate::endpoints::EndpointConfig;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum Asset {
    #[constrainable]
    StaticDataTable(StaticDataTable),
}
impl Asset {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> String {
        match self {
            Asset::StaticDataTable(x) => x.get_presto_schemas(templates, endpoints),
        }
    }
}
