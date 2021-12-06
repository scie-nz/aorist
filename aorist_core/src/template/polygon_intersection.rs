#![allow(non_snake_case)]
use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct PolygonIntersection {
    pub name: AString,
    pub dimensions: usize,
    pub crs: Option<usize>,
    pub binary_only: bool,
}
impl TDatumTemplate for PolygonIntersection {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        Vec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
