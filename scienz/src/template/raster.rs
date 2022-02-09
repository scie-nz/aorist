use abi_stable::std_types::ROption;
use aorist_util::AOption;

use crate::attributes::*;

use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
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
