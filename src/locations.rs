use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GCSLocation {
    uri: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum RemoteWebsiteLocation {
    GCSLocation(GCSLocation),
}


