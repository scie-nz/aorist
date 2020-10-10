use serde::{Serialize, Deserialize};
use std::fs;
/* This trait will implement all the abstract operations
 * we might want to undertake on a regular dataset.
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GCSLocation {
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "properties")]
pub enum Location {
    GCSLocation { uri: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticCSVDataset {
    pub name: String,
    pub location: GCSLocation,
}


#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "properties")]
pub enum Dataset {
    StaticCSVDataset { name: String, location: Location },
}
impl Dataset {
    pub fn to_yaml(&self) -> String {
        match self {
            Dataset::StaticCSVDataset{..} => serde_yaml::to_string(self).unwrap(),
            _ => "Error converting to yaml.".to_string(),
        }
    }
}

pub fn get_dataset() -> Dataset {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let dataset: Dataset = serde_yaml::from_str(&s).unwrap();
    dataset
}
