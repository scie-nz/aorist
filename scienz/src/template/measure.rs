use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::attributes::*;

use crate::schema::TabularSchema;
use crate::template::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_attributes::{Count, FloatPrediction, Regressor};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AUuid;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// An integer-valued measure for the density of attribute
/// combinations. For example, a count in a histogram.
/// Note: the measure name is also used as the column name
/// in a table.
#[aorist]
pub struct IntegerMeasure {
    pub name: AString,
    pub comment: AOption<AString>,
}

impl TDatumTemplate for IntegerMeasure {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
#[aorist]
pub struct TrainedFloatMeasure {
    pub name: AString,
    pub comment: AOption<AString>,
}
impl TDatumTemplate for TrainedFloatMeasure {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}

#[aorist]
pub struct PredictionsFromTrainedFloatMeasure {
    pub name: AString,
    pub comment: AOption<AString>,
}
impl PredictionsFromTrainedFloatMeasure {
    pub fn get_name(&self) -> AString {
        self.name.clone()
    }
    pub fn get_model_asset_role(&self) -> String {
        "model".into()
    }
    pub fn get_source_asset_role(&self) -> String {
        "source".into()
    }
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        AVec::new()
    }
}
