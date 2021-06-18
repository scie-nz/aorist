#![allow(dead_code)]
use crate::schema::TabularSchema;
use crate::template::datum_template::TDatumTemplate;
use aorist_attributes::{
    Attribute, AttributeEnum, AttributeOrTransform, Count, FloatPrediction, Regressor,
};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
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
    pub attributes: Vec<Attribute>,
    pub source_asset_name: String,
}

impl TDatumTemplate for IntegerMeasure {
    fn get_attributes(&self) -> Vec<Attribute> {
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
    pub fn get_frequency_attribute(&self) -> Attribute {
        Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::Count(Count {
                name: self.name.clone(),
                comment: self.comment.clone(),
                nullable: false,
            })),
            tag: None,
            uuid: None,
        }
    }
}
#[aorist]
pub struct TrainedFloatMeasure {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub features: Vec<Attribute>,
    #[constrainable]
    pub objective: Attribute,
    pub source_asset_name: String,
}

impl TDatumTemplate for TrainedFloatMeasure {
    fn get_attributes(&self) -> Vec<Attribute> {
        let mut attr = self.features.clone();
        let prediction_attribute = self.get_prediction_attribute();
        attr.push(prediction_attribute);
        attr.push(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::Regressor(
                self.get_regressor_as_attribute().clone(),
            )),
            tag: None,
            uuid: None,
        });

        attr
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl TrainedFloatMeasure {
    pub fn get_prediction_attribute(&self) -> Attribute {
        Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::FloatPrediction(
                FloatPrediction {
                    name: self.name.clone(),
                    comment: self.comment.clone(),
                    nullable: false,
                },
            )),
            tag: None,
            uuid: None,
        }
    }
    pub fn get_training_objective(&self) -> Attribute {
        self.objective.clone()
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
            datumTemplateName: self.name.clone(),
            attributes: self
                .features
                .iter()
                .map(|x| x.inner.get_name().clone())
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
    pub features: Vec<Attribute>,
    #[constrainable]
    pub objective: Attribute,
}
impl PredictionsFromTrainedFloatMeasure {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_attributes(&self) -> Vec<Attribute> {
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