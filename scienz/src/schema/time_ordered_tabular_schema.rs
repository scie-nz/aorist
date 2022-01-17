use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct TimeOrderedTabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub attributes: AVec<AString>,
    // non-null time stamp columns used to order records
    // order is always: 1st column, then 2nd, etc.
    pub ordering_attributes: AVec<AString>,
}
impl TimeOrderedTabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
