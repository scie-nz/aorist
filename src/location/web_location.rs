use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use markdown_gen::markdown::*;

#[aorist_concept]
pub struct WebLocation {
    // TODO: replace these with Getters and Setters
    pub address: String,
}

impl WebLocation {
    pub fn markdown(&self, md: &mut Markdown<Vec<u8>>) {
        md.write(
            List::new(true)
                .title("WebLocation")
                .item("address".bold().paragraph().append(": ").append(&*self.address))
        ).unwrap();
    }
}
