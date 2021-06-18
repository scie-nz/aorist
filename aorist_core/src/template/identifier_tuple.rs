use aorist_attributes::Attribute;
use aorist_primitives::{AoristConcept, ConceptEnum};

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
