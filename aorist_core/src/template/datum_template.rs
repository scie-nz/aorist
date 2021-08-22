use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::template::filter::*;
use crate::template::identifier_tuple::*;
use crate::template::measure::*;
use crate::template::row_struct::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

pub trait TDatumTemplate {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>>;
    fn get_name(&self) -> String;
}

#[aorist]
pub enum DatumTemplate {
    TrainedFloatMeasure(AoristRef<TrainedFloatMeasure>),
    PredictionsFromTrainedFloatMeasure(AoristRef<PredictionsFromTrainedFloatMeasure>),
    RowStruct(AoristRef<RowStruct>),
    IdentifierTuple(AoristRef<IdentifierTuple>),
    IntegerMeasure(AoristRef<IntegerMeasure>),
    Filter(AoristRef<Filter>),
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
            DatumTemplate::RowStruct(x) => x.0.read().unwrap().get_name(),
            DatumTemplate::IdentifierTuple(x) => x.0.read().unwrap().get_name(),
            DatumTemplate::IntegerMeasure(x) => x.0.read().unwrap().get_name(),
            DatumTemplate::TrainedFloatMeasure(x) => x.get_name(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.0.read().unwrap().get_name(),
            DatumTemplate::Filter(x) => x.0.read().unwrap().get_name(),
        }
    }
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        match self {
            DatumTemplate::RowStruct(x) => x.0.read().unwrap().get_attributes(),
            DatumTemplate::IdentifierTuple(x) => x.0.read().unwrap().get_attributes(),
            DatumTemplate::IntegerMeasure(x) => x.0.read().unwrap().get_attributes(),
            DatumTemplate::TrainedFloatMeasure(x) => x.get_attributes(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => {
                x.0.read().unwrap().get_attributes()
            }
            DatumTemplate::Filter(x) => x.0.read().unwrap().get_attributes(),
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyDatumTemplate {
    pub fn attributes(&self) -> PyResult<Vec<PyAttribute>> {
        Ok(self
            .inner
            .0
            .read()
            .unwrap()
            .get_attributes()
            .into_iter()
            .map(|x| PyAttribute { inner: x })
            .collect())
    }
}
