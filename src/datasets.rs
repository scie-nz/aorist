use serde::{Serialize, Deserialize};
use std::fs;
/* This trait will implement all the abstract operations
 * we might want to undertake on a regular dataset.
 */
pub trait Dataset {
    fn to_yaml(&self) -> String;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticCSVDataset {
    pub x: f64,
    pub y: f64,
    pub name: String,
}

impl Dataset for StaticCSVDataset {
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "properties")]
pub enum DatasetEnum {
    StaticCSVDataset { x: f64, y: f64, name: String },
}

pub fn get_dataset() -> Option<Box<dyn Dataset>> {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let dataset: DatasetEnum = serde_yaml::from_str(&s).unwrap();
    match dataset {
        DatasetEnum::StaticCSVDataset{ x, y, name } => Some(Box::new(StaticCSVDataset{ x, y, name })),
        _ => None
    }
}
