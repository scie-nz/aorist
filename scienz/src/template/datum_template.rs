use crate::attributes::*;

use crate::template::filter::*;
use crate::template::identifier_tuple::*;
use crate::template::measure::*;
use crate::template::point_cloud::*;
use crate::template::point_cloud_info::*;
use crate::template::polygon::*;
use crate::template::polygon_intersection::*;
use crate::template::raster::*;
use crate::template::row_struct::*;
use crate::template::tensor::*;
use crate::template::text::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait TDatumTemplate {
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>>;
    fn get_name(&self) -> AString;
}

#[aorist]
pub enum DatumTemplate {
    TrainedFloatMeasure(AoristRef<TrainedFloatMeasure>),
    PredictionsFromTrainedFloatMeasure(AoristRef<PredictionsFromTrainedFloatMeasure>),
    RowStruct(AoristRef<RowStruct>),
    IdentifierTuple(AoristRef<IdentifierTuple>),
    IntegerMeasure(AoristRef<IntegerMeasure>),
    Filter(AoristRef<Filter>),
    Tensor(AoristRef<Tensor>),
    PointCloud(AoristRef<PointCloud>),
    PointCloudInfo(AoristRef<PointCloudInfo>),
    Polygon(AoristRef<Polygon>),
    PolygonIntersection(AoristRef<PolygonIntersection>),
    Raster(AoristRef<Raster>),
    Text(AoristRef<Text>),
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
            DatumTemplate::Tensor(_) => "Tensor",
            DatumTemplate::PointCloud(_) => "PointCloud",
            DatumTemplate::PointCloudInfo(_) => "PointCloudInfo",
            DatumTemplate::Polygon(_) => "Polygon",
            DatumTemplate::PolygonIntersection(_) => "PolygonIntersection",
            DatumTemplate::Text(_) => "Text",
            DatumTemplate::Raster(_) => "Raster",
        }
        .to_string()
    }
}
impl TDatumTemplate for DatumTemplate {
    fn get_name(&self) -> AString {
        match self {
            DatumTemplate::RowStruct(x) => x.0.read().get_name(),
            DatumTemplate::IdentifierTuple(x) => x.0.read().get_name(),
            DatumTemplate::IntegerMeasure(x) => x.0.read().get_name(),
            DatumTemplate::TrainedFloatMeasure(x) => x.0.read().get_name(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.0.read().get_name(),
            DatumTemplate::Filter(x) => x.0.read().get_name(),
            DatumTemplate::Tensor(x) => x.0.read().get_name(),
            DatumTemplate::PointCloud(x) => x.0.read().get_name(),
            DatumTemplate::PointCloudInfo(x) => x.0.read().get_name(),
            DatumTemplate::Polygon(x) => x.0.read().get_name(),
            DatumTemplate::PolygonIntersection(x) => x.0.read().get_name(),
            DatumTemplate::Text(x) => x.0.read().get_name(),
            DatumTemplate::Raster(x) => x.0.read().get_name(),
        }
    }
    fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        match self {
            DatumTemplate::RowStruct(x) => x.0.read().get_attributes(),
            DatumTemplate::IdentifierTuple(x) => x.0.read().get_attributes(),
            DatumTemplate::IntegerMeasure(x) => x.0.read().get_attributes(),
            DatumTemplate::TrainedFloatMeasure(x) => x.0.read().get_attributes(),
            DatumTemplate::PredictionsFromTrainedFloatMeasure(x) => x.0.read().get_attributes(),
            DatumTemplate::Filter(x) => x.0.read().get_attributes(),
            DatumTemplate::Tensor(x) => x.0.read().get_attributes(),
            DatumTemplate::PointCloud(x) => x.0.read().get_attributes(),
            DatumTemplate::PointCloudInfo(x) => x.0.read().get_attributes(),
            DatumTemplate::Polygon(x) => x.0.read().get_attributes(),
            DatumTemplate::PolygonIntersection(x) => x.0.read().get_attributes(),
            DatumTemplate::Text(x) => x.0.read().get_attributes(),
            DatumTemplate::Raster(x) => x.0.read().get_attributes(),
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
            .get_attributes()
            .into_iter()
            .map(|x| PyAttribute { inner: x })
            .collect())
    }
    #[getter]
    pub fn get_name(&self) -> AString {
        self.inner.0.read().get_name()
    }
}
