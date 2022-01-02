use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::error::AoristError;
use crate::schema::geospatial_asset_schema::*;
use crate::schema::graph_asset_schema::*;
use crate::schema::language_asset_schema::*;
use crate::schema::long_tabular_schema::*;
use crate::schema::tabular_collection_schema::*;
use crate::schema::tabular_schema::*;
use crate::schema::time_ordered_tabular_schema::*;
use crate::schema::undefined_tabular_schema::*;
use crate::schema::vision_asset_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum DataSchema {
    #[constrainable]
    GeospatialAssetSchema(AoristRef<GeospatialAssetSchema>),
    #[constrainable]
    GraphAssetSchema(AoristRef<GraphAssetSchema>),
    #[constrainable]
    LanguageAssetSchema(AoristRef<LanguageAssetSchema>),
    #[constrainable]
    LongTabularSchema(AoristRef<LongTabularSchema>),
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    TimeOrderedTabularSchema(AoristRef<TimeOrderedTabularSchema>),
    #[constrainable]
    UndefinedTabularSchema(AoristRef<UndefinedTabularSchema>),
    #[constrainable]
    TabularCollectionSchema(AoristRef<TabularCollectionSchema>),
    #[constrainable]
    VisionAssetSchema(AoristRef<VisionAssetSchema>),
}

impl DataSchema {
    pub fn get_datum_template(&self) -> Result<AoristRef<DatumTemplate>, AoristError> {
        match self {
            DataSchema::TabularSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::TabularCollectionSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::LongTabularSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::LanguageAssetSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::VisionAssetSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::GeospatialAssetSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::GraphAssetSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::TimeOrderedTabularSchema(x) => Ok(x.0.read().get_datum_template().clone()),
            DataSchema::UndefinedTabularSchema(_) => Err(AoristError::OtherError(AString::from(
                "UndefinedTabularSchema has no datum template.",
            ))),
        }
    }
    pub fn get_datum_template_name(&self) -> Result<AString, AoristError> {
        match self {
            DataSchema::TabularSchema(x) => Ok(x.0.read().get_datum_template().0.read().get_name()),
            DataSchema::TabularCollectionSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::GraphAssetSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::LongTabularSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::LanguageAssetSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::VisionAssetSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::GeospatialAssetSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::TimeOrderedTabularSchema(x) => {
                Ok(x.0.read().get_datum_template().0.read().get_name())
            }
            DataSchema::UndefinedTabularSchema(_) => Err(AoristError::OtherError(AString::from(
                "UndefinedTabularSchema has no datum template.",
            ))),
        }
    }
    pub fn get_attribute_names(&self) -> AVec<AString> {
        match self {
            DataSchema::TabularSchema(x) => x.0.read().attributes.clone(),
            DataSchema::TabularCollectionSchema(x) => x.0.read().attributes.clone(),
            DataSchema::LongTabularSchema(x) => x.0.read().get_attribute_names(),
            DataSchema::VisionAssetSchema(x) => {
                x.0.read()
                    .get_attributes()
                    .iter()
                    .map(|x| x.0.read().get_name())
                    .collect()
            }
            DataSchema::LanguageAssetSchema(x) => {
                x.0.read()
                    .get_attributes()
                    .iter()
                    .map(|x| x.0.read().get_name())
                    .collect()
            }
            DataSchema::GeospatialAssetSchema(x) => {
                x.0.read()
                    .get_attributes()
                    .iter()
                    .map(|x| x.0.read().get_name())
                    .collect()
            }
            DataSchema::GraphAssetSchema(x) => {
                x.0.read()
                    .get_attributes()
                    .iter()
                    .map(|x| x.0.read().get_name())
                    .collect()
            }
            DataSchema::TimeOrderedTabularSchema(x) => x.0.read().attributes.clone(),
            DataSchema::UndefinedTabularSchema(_) => vec![].into_iter().collect(),
        }
    }
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        match self {
            DataSchema::GeospatialAssetSchema(x) => x.0.read().get_attributes(),
            DataSchema::GraphAssetSchema(x) => x.0.read().get_attributes(),
            DataSchema::TabularCollectionSchema(x) => x.0.read().get_attributes(),
            _ => self.get_datum_template().unwrap().0.read().get_attributes(),
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyDataSchema {
    pub fn get_datum_template_name(&self) -> PyResult<AString> {
        match self.inner.0.read().get_datum_template_name() {
            Ok(s) => Ok(s),
            Err(err) => Err(PyValueError::new_err(err.as_str().to_string())),
        }
    }
    #[getter]
    pub fn get_datum_template(&self) -> PyResult<PyDatumTemplate> {
        match self.inner.0.read().get_datum_template() {
            Ok(s) => Ok(PyDatumTemplate { inner: s.clone() }),
            Err(err) => Err(PyValueError::new_err(err.as_str().to_string())),
        }
    }
    #[getter]
    pub fn get_attributes(&self) -> Vec<PyAttribute> {
        self.inner
            .0
            .read()
            .get_attributes()
            .iter()
            .map(|x| PyAttribute { inner: x.clone() })
            .collect()
    }
}
