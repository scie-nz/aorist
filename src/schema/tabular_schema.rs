#![allow(non_snake_case)]
use crate::attributes::Attribute;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::template::{DatumTemplate, TDatumTemplate};
use aorist_primitives::TAttribute;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use aorist_concept::{aorist_concept, Constrainable};

#[aorist_concept]
pub struct TabularSchema {
    pub datumTemplateName: String,
    pub attributes: Vec<String>,
}

#[pymethods]
impl TabularSchema {
    #[new]
    fn new(datumTemplate: DatumTemplate, attributes_opt: Option<Vec<Attribute>>) -> Self {
        if let Some(attributes) = attributes_opt {
            Self {
                datumTemplateName: datumTemplate.get_name(),
                attributes: attributes.iter().map(|x| x.get_name().clone()).collect(),
                uuid: None,
                tag: None,
                constraints: Vec::new(),
            }
        } else {
            Self {
                datumTemplateName: datumTemplate.get_name(),
                attributes: datumTemplate
                    .get_attributes()
                    .iter()
                    .map(|x| x.get_name().clone())
                    .collect(),
                uuid: None,
                tag: None,
                constraints: Vec::new(),
            }
        }
    }
}
