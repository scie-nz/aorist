use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct RowStruct {
    pub name: AString,
    #[constrainable]
    pub attributes: AVec<AoristRef<Attribute>>,
}
impl TDatumTemplate for RowStruct {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
