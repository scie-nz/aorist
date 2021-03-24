#![allow(non_snake_case)]

use crate::attributes::*;
use crate::concept::{AoristConcept, Concept};
use crate::template::filter::*;
use crate::template::identifier_tuple::*;
use crate::template::keyed_struct::*;
use crate::template::measure::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum DatumTemplate {
    TrainedFloatMeasure(TrainedFloatMeasure),
    PredictionsFromTrainedFloatMeasure(PredictionsFromTrainedFloatMeasure),
    RowStruct(RowStruct),
    IdentifierTuple(IdentifierTuple),
    IntegerMeasure(IntegerMeasure),
    Filter(Filter),
}
impl DatumTemplate {
    pub fn get_type(&self) -> String {
        match self {
            DatumTemplate::RowStruct(_) => "RowStruct",
            DatumTemplate::IdentifierTuple(_) => "IdentifierTuple",
            DatumTemplate::IntegerMeasure(_) => "IntegerMeasure",
            DatumTemplate::TrainedFloatMeasure(_) => "TrainedFloatMeasure",
            DatumTemplate::PredictionsFromTrainedFloatMeasure(_) => {
                "PredictionsFromTrainedFloatMeasure"
            }
            DatumTemplate::Filter(_) => "Filter",
        }
        .to_string()
    }
}
pub trait TDatumTemplate {
    fn get_attributes(&self) -> Vec<Attribute>;
    fn get_name(&self) -> String;
}
pub trait TInnerDatumTemplate {
    fn get_attributes(&self) -> Vec<InnerAttribute>;
    fn get_name(&self) -> String;
}
impl TDatumTemplate for DatumTemplate {
    fn get_name(&self) -> String {
        match self {
            DatumTemplate::RowStruct(x) => x.get_name(),
            DatumTemplate::IdentifierTuple(x) => x.get_name(),
            DatumTemplate::IntegerMeasure(x) => x.get_name(),
            DatumTemplate::TrainedFloatMeasure(x) => x.get_name(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.get_name(),
            DatumTemplate::Filter(x) => x.get_name(),
        }
    }
    fn get_attributes(&self) -> Vec<Attribute> {
        match self {
            DatumTemplate::RowStruct(x) => x.get_attributes(),
            DatumTemplate::IdentifierTuple(x) => x.get_attributes(),
            DatumTemplate::IntegerMeasure(x) => x.get_attributes(),
            DatumTemplate::TrainedFloatMeasure(x) => x.get_attributes(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.get_attributes(),
            DatumTemplate::Filter(x) => x.get_attributes(),
        }
    }
}
impl InnerDatumTemplate {
    pub fn get_name(&self) -> String {
        match self {
            InnerDatumTemplate::RowStruct(x) => x.name.clone(),
            InnerDatumTemplate::IdentifierTuple(x) => x.name.clone(),
            InnerDatumTemplate::IntegerMeasure(x) => x.name.clone(),
            InnerDatumTemplate::TrainedFloatMeasure(x) => x.name.clone(),
            InnerDatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.name.clone(),
            InnerDatumTemplate::Filter(x) => x.name.clone(),
        }
    }
    pub fn get_attributes(&self) -> Vec<InnerAttribute> {
        match self {
            InnerDatumTemplate::RowStruct(x) => x.attributes.clone(),
            InnerDatumTemplate::IdentifierTuple(x) => x.attributes.clone(),
            _ => panic!("Not implemented"),
        }
    }
}
