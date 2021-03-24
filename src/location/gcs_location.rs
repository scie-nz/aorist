use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use markdown_gen::markdown::*;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct GCSLocation {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub blob: String,
}

impl GCSLocation {
    pub fn markdown(&self, md: &mut Markdown<Vec<u8>>) {
        md.write(
            List::new(true)
                .title("GCSLocation")
                .item(
                    "bucket"
                        .bold()
                        .paragraph()
                        .append(": ")
                        .append(&*self.bucket),
                )
                .item("blob".bold().paragraph().append(": ").append(&*self.blob)),
        )
        .unwrap();
    }
}
