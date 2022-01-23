use abi_stable::std_types::ROption;
use aorist_util::AOption;

use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct PolygonIntersection {
    pub name: AString,
    pub dimensions: usize,
    pub crs: AOption<usize>,
    pub binary_only: bool,
}
impl TDatumTemplate for PolygonIntersection {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
