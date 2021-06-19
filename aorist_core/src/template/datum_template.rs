use crate::attributes::*;
use crate::template::filter::*;
use crate::template::identifier_tuple::*;
use crate::template::measure::*;
use crate::template::row_struct::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait TDatumTemplate {
    fn get_attributes(&self) -> Vec<Attribute>;
    fn get_name(&self) -> String;
}

#[aorist]
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
