use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::location::gcs_location::*;
use crate::location::web_location::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
    #[constrainable]
    WebLocation(WebLocation),
}
