use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::location::gcs_location::GCSLocation;


#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum RemoteLocation {
    GCSLocation(GCSLocation),
}
