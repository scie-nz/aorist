use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::template::datum_template::TDatumTemplate;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

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
