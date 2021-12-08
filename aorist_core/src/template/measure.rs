
#![allow(dead_code)]
use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::schema::TabularSchema;
use crate::template::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_attributes::{Count, FloatPrediction, Regressor};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

/// An integer-valued measure for the density of attribute
/// combinations. For example, a count in a histogram.
/// Note: the measure name is also used as the column name
/// in a table.
#[aorist]
pub struct IntegerMeasure {
    pub name: AString,
    pub comment: Option<AString>,
    #[constrainable]
    pub attributes: AVec<AoristRef<Attribute>>,
    pub source_asset_name: AString,
}

impl TDatumTemplate for IntegerMeasure {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        let mut attr = self.attributes.clone();
        let frequency_attribute = self.get_frequency_attribute();
        attr.push(frequency_attribute);
        attr
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
impl IntegerMeasure {
    pub fn get_frequency_attribute(&self) -> AoristRef<Attribute> {
        AoristRef(RArc::new(RRwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::Count(Count {
                name: self.name.clone(),
                comment: self.comment.clone(),
                nullable: false,
            })),
            tag: None,
            uuid: None,
        })))
    }
}
#[aorist]
pub struct TrainedFloatMeasure {
    pub name: AString,
    pub comment: Option<AString>,
    #[constrainable]
    pub features: AVec<AoristRef<Attribute>>,
    #[constrainable]
    pub objective: AoristRef<Attribute>,
    pub source_asset_name: AString,
}

impl TDatumTemplate for AoristRef<TrainedFloatMeasure> {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        let mut attr = self.0.read().features.clone();
        let prediction_attribute = self.get_prediction_attribute();
        attr.push(prediction_attribute);
        attr.push(AoristRef(RArc::new(RRwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::Regressor(
                self.get_regressor_as_attribute().clone(),
            )),
            tag: None,
            uuid: None,
        }))));

        attr
    }
    fn get_name(&self) -> AString {
        self.0.read().name.clone()
    }
}
impl AoristRef<TrainedFloatMeasure> {
    pub fn get_prediction_attribute(&self) -> AoristRef<Attribute> {
        AoristRef(RArc::new(RRwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::FloatPrediction(
                FloatPrediction {
                    name: self.0.read().name.clone(),
                    comment: self.0.read().comment.clone(),
                    nullable: false,
                },
            )),
            tag: None,
            uuid: None,
        })))
    }
    pub fn get_training_objective(&self) -> AoristRef<Attribute> {
        self.0.read().objective.clone()
    }
    pub fn get_regressor_as_attribute(&self) -> Regressor {
        Regressor {
            name: "model".into(),
            comment: Some("A serialized version of the model".into()),
            nullable: false,
        }
    }
    pub fn get_model_storage_tabular_schema(&self) -> TabularSchema {
        TabularSchema {
            datum_template: AoristRef(RArc::new(RRwLock::new(DatumTemplate::TrainedFloatMeasure(
                self.clone(),
            )))),
            attributes: self
                .0
                .read()
                .features
                .iter()
                .map(|x| x.0.read().inner.get_name().clone())
                .collect(),
            tag: None,
            uuid: None,
        }
    }
}

#[aorist]
pub struct PredictionsFromTrainedFloatMeasure {
    pub name: AString,
    pub comment: Option<AString>,
    #[constrainable]
    pub features: AVec<AoristRef<Attribute>>,
    #[constrainable]
    pub objective: AoristRef<Attribute>,
}
impl PredictionsFromTrainedFloatMeasure {
    pub fn get_name(&self) -> AString {
        self.name.clone()
    }
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        let mut attr = self.features.clone();
        let prediction_attribute = self.objective.clone();
        attr.push(prediction_attribute);
        attr
    }
    pub fn get_model_asset_role(&self) -> String {
        "model".into()
    }
    pub fn get_source_asset_role(&self) -> String {
        "source".into()
    }
}
