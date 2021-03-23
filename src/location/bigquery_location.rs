use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use markdown_gen::markdown::*;

#[aorist_concept]
pub struct BigQueryLocation {}

impl BigQueryLocation {
    pub fn markdown(&self, md: &mut Markdown<Vec<u8>>) {
        md.write("BigQueryLocation".bold().paragraph().append(
            ": the data resides in BigQuery."
        )).unwrap();
    }
}
