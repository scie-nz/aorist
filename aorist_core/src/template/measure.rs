#![allow(dead_code)]
use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::schema::TabularSchema;
use crate::template::*;
use aorist_attributes::{Count, FloatPrediction, Regressor};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// An integer-valued measure for the density of attribute
/// combinations. For example, a count in a histogram.
/// Note: the measure name is also used as the column name
/// in a table.
#[aorist]
pub struct IntegerMeasure {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub attributes: Vec<AoristRef<Attribute>>,
    pub source_asset_name: String,
}

impl TDatumTemplate for IntegerMeasure {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        let mut attr = self.attributes.clone();
        let frequency_attribute = self.get_frequency_attribute();
        attr.push(frequency_attribute);
        attr
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl IntegerMeasure {
    pub fn get_frequency_attribute(&self) -> AoristRef<Attribute> {
        AoristRef(Arc::new(RwLock::new(Attribute {
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
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub features: Vec<AoristRef<Attribute>>,
    #[constrainable]
    pub objective: AoristRef<Attribute>,
    pub source_asset_name: String,
}

impl TDatumTemplate for AoristRef<TrainedFloatMeasure> {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        let mut attr = self.0.read().unwrap().features.clone();
        let prediction_attribute = self.get_prediction_attribute();
        attr.push(prediction_attribute);
        attr.push(AoristRef(Arc::new(RwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::Regressor(
                self.get_regressor_as_attribute().clone(),
            )),
            tag: None,
            uuid: None,
        }))));

        attr
    }
    fn get_name(&self) -> String {
        self.0.read().unwrap().name.clone()
    }
}
impl AoristRef<TrainedFloatMeasure> {
    pub fn get_prediction_attribute(&self) -> AoristRef<Attribute> {
        AoristRef(Arc::new(RwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::FloatPrediction(
                FloatPrediction {
                    name: self.0.read().unwrap().name.clone(),
                    comment: self.0.read().unwrap().comment.clone(),
                    nullable: false,
                },
            )),
            tag: None,
            uuid: None,
        })))
    }
    pub fn get_training_objective(&self) -> AoristRef<Attribute> {
        self.0.read().unwrap().objective.clone()
    }
    pub fn get_regressor_as_attribute(&self) -> Regressor {
        Regressor {
            name: "model".to_string(),
            comment: Some("A serialized version of the model".to_string()),
            nullable: false,
        }
    }
    pub fn get_model_storage_tabular_schema(&self) -> TabularSchema {
        TabularSchema {
            datum_template: AoristRef(Arc::new(RwLock::new(DatumTemplate::TrainedFloatMeasure(
                self.clone(),
            )))),
            attributes: self
                .0
                .read()
                .unwrap()
                .features
                .iter()
                .map(|x| x.0.read().unwrap().inner.get_name().clone())
                .collect(),
            tag: None,
            uuid: None,
        }
    }
}

#[aorist]
pub struct PredictionsFromTrainedFloatMeasure {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub features: Vec<AoristRef<Attribute>>,
    #[constrainable]
    pub objective: AoristRef<Attribute>,
}
impl PredictionsFromTrainedFloatMeasure {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        let mut attr = self.features.clone();
        let prediction_attribute = self.objective.clone();
        attr.push(prediction_attribute);
        attr
    }
    pub fn get_model_asset_role(&self) -> String {
        "model".to_string()
    }
    pub fn get_source_asset_role(&self) -> String {
        "source".to_string()
    }
}
