#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::encoding::csv_encoding::*;
use crate::encoding::orc_encoding::*;
use crate::encoding::tsv_encoding::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
    TSVEncoding(TSVEncoding),
}
