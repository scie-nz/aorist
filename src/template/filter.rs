#![allow(non_snake_case)]

use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::Constraint;
use crate::template::TDatumTemplate;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct Filter {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<Attribute>,
    #[constrainable]
    pub predicate: Option<Predicate>,
    pub source_asset_name: String,
}
impl TDatumTemplate for Filter {
    fn get_attributes(&self) -> Vec<Attribute> {
        self.attributes.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
