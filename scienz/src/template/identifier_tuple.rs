use crate::attributes::*;
use abi_stable::std_types::ROption;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AoristRef;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct IdentifierTuple {
    pub name: AString,
    #[constrainable]
    pub attributes: AVec<AoristRef<Attribute>>,
}
impl TDatumTemplate for IdentifierTuple {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
