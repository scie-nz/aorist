use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::location::gcs_location::GCSLocation;
use crate::location::web_location::WebLocation;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
    #[constrainable]
    WebLocation(WebLocation),
}
