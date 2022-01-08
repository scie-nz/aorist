use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct PointCloud {
    pub name: AString,
    pub dimensions: usize,
    pub horiz_crs: AOption<usize>,
    pub vert_crs: AOption<usize>,
}
impl TDatumTemplate for PointCloud {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}