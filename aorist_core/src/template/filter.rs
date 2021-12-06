use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::predicate::*;
use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct Filter {
    pub name: AString,
    #[constrainable]
    pub attributes: Vec<AoristRef<Attribute>>,
    #[constrainable]
    pub predicate: Option<AoristRef<Predicate>>,
    pub source_asset_name: AString,
}
impl TDatumTemplate for Filter {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
