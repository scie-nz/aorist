use crate::attributes::*;

use crate::predicate::*;
use crate::template::datum_template::TDatumTemplate;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use aorist_primitives::AUuid;

#[aorist]
pub struct Filter {
    pub name: AString,
    #[constrainable]
    pub attributes: AVec<AoristRef<Attribute>>,
    #[constrainable]
    pub predicate: AOption<AoristRef<Predicate>>,
    pub source_asset_name: AString,
}
impl TDatumTemplate for Filter {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
