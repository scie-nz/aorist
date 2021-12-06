use crate::attributes::*;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};

use crate::concept::{AoristRef, WrappedConcept};
use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct IdentifierTuple {
    pub name: AString,
    #[constrainable]
    pub attributes: Vec<AoristRef<Attribute>>,
}
impl TDatumTemplate for IdentifierTuple {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
