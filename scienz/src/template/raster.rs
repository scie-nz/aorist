use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AUuid;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct Raster {
    pub name: AString,
    pub dimensions: usize,
    pub epsg: AOption<usize>,
    pub channels: AOption<usize>,
}
impl TDatumTemplate for Raster {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
