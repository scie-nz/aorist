use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::predicate::*;
use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct Filter {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<AoristRef<Attribute>>,
    #[constrainable]
    pub predicate: Option<AoristRef<Predicate>>,
    pub source_asset_name: String,
}
impl TDatumTemplate for Filter {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
