use aorist_attributes::Attribute;
use aorist_primitives::{AoristConcept, ConceptEnum};

use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::template::datum_template::TDatumTemplate;

#[aorist]
pub struct IdentifierTuple {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<Attribute>,
}
impl TDatumTemplate for IdentifierTuple {
    fn get_attributes(&self) -> Vec<Attribute> {
        self.attributes.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
