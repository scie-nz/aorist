#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::encoding::csv_encoding::CSVEncoding;
use crate::encoding::orc_encoding::ORCEncoding;
use crate::encoding::tsv_encoding::TSVEncoding;
use aorist_concept::{aorist_concept2, Constrainable};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
    TSVEncoding(TSVEncoding),
}
