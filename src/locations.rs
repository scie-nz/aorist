use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GCSLocation {
    uri: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum RemoteWebsiteLocation {
    GCSLocation(GCSLocation),
}


