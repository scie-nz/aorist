use crate::predicate::Predicate;
use crate::template::datum_template::TDatumTemplate;
use aorist_attributes::Attribute;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
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
