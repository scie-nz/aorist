use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct RasterSchema {
    pub datum_template: AoristRef<DatumTemplate>,
}
impl RasterSchema {
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        self.datum_template.0.read().get_attributes()
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
