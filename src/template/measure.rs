#![allow(non_snake_case)]

use crate::attributes::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::template::datum_template::TDatumTemplate;

use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// An integer-valued measure for the density of attribute
/// combinations. For example, a count in a histogram.
/// Note: the measure name is also used as the column name
/// in a table.
#[aorist_concept]
pub struct IntegerMeasure {
    pub name: String,
    #[py_default = "None"]
    pub comment: Option<String>,
    #[constrainable]
    pub attributes: Vec<Attribute>,
    source_asset_name: String,
}

impl TDatumTemplate for IntegerMeasure {
    fn get_attributes(&self) -> Vec<Attribute> {
        let mut attr = self.attributes.clone();
        let frequency_attribute = Attribute::Count(Count {
            name: self.name.clone(),
            comment: self.comment.clone(),
            tag: None,
            uuid: None,
            constraints: Vec::new(),
        });
        attr.push(frequency_attribute);
        attr
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
