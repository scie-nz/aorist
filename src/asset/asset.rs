#![allow(non_snake_case)]
use crate::asset::static_data_table::StaticDataTable;
use crate::endpoints::EndpointConfig;
use crate::template::DatumTemplate;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

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
