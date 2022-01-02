use crate::attributes::*;
use aorist_primitives::AoristRef;
use crate::concept::WrappedConcept;
use crate::template::datum_template::TDatumTemplate;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConceptBase, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct Tensor {
    pub name: AString,
    pub dimensions: usize,
    #[constrainable]
    pub attributes: AVec<AoristRef<Attribute>>,
}
impl TDatumTemplate for Tensor {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
