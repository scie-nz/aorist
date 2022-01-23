use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct PointCloudInfo {
    pub name: AString,
    pub metadata: bool,
    pub boundaries: bool,
}
impl TDatumTemplate for PointCloudInfo {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        // TODO: fill this in
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
