#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::file_based_storage_layout::*;
use super::api_layout::*;


#[aorist_concept]
pub enum APIOrFileLayout {
    FileBasedStorageLayout(FileBasedStorageLayout),
    APILayout(APILayout),
}
