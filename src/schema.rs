#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TabularSchema {
    datumTemplateName: String,
    attributes: Vec<String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum DataSchema {
    TabularSchema(TabularSchema),
}
