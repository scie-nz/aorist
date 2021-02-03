#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::AoristConstraint;
use crate::constraint::Constraint;
use crate::encoding::Encoding;
use crate::layout::FileBasedStorageLayout;
use crate::location::RemoteLocation;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct RemoteStorage {
    #[constrainable]
    location: RemoteLocation,
    #[constrainable]
    layout: FileBasedStorageLayout,
    #[constrainable]
    encoding: Encoding,
}
