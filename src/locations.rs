use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Location {
    GCSLocation { uri: String },
}

