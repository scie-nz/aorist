use crate::concept::{AoristConcept, Concept};
use crate::location::gcs_location::*;
use crate::location::web_location::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
    #[constrainable]
    WebLocation(WebLocation),
}
