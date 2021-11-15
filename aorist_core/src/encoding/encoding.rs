#![allow(unused_parens)]
use crate::compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::csv_encoding::*;
use crate::encoding::gdb_encoding::*;
use crate::encoding::geotiff_encoding::*;
use crate::encoding::gpkg_encoding::*;
use crate::encoding::json_encoding::*;
use crate::encoding::kml_encoding::*;
use crate::encoding::las_encoding::*;
use crate::encoding::onnx_encoding::*;
use crate::encoding::orc_encoding::*;
use crate::encoding::shapefile_encoding::*;
use crate::encoding::sqlite_encoding::*;
use crate::encoding::tiff_encoding::*;
use crate::encoding::tsv_encoding::*;
use crate::encoding::wkt_encoding::*;
use crate::encoding::xml_encoding::*;
use crate::header::FileHeader;
use crate::header::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum Encoding {
    CSVEncoding(AoristRef<CSVEncoding>),
    JSONEncoding(AoristRef<JSONEncoding>),
    NewlineDelimitedJSONEncoding(AoristRef<NewlineDelimitedJSONEncoding>),
    ORCEncoding(AoristRef<ORCEncoding>),
    TSVEncoding(AoristRef<TSVEncoding>),
    ONNXEncoding(AoristRef<ONNXEncoding>),
    GDBEncoding(AoristRef<GDBEncoding>),
    LASEncoding(AoristRef<LASEncoding>),
    SQLiteEncoding(AoristRef<SQLiteEncoding>),
    GeoTiffEncoding(AoristRef<GeoTiffEncoding>),
    TiffEncoding(AoristRef<TiffEncoding>),
    WKTEncoding(AoristRef<WKTEncoding>),
    ShapefileEncoding(AoristRef<ShapefileEncoding>),
    XMLEncoding(AoristRef<XMLEncoding>),
    KMLEncoding(AoristRef<KMLEncoding>),
    GPKGEncoding(AoristRef<GPKGEncoding>),
}

impl Encoding {
    pub fn get_header(&self) -> Option<AoristRef<FileHeader>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().unwrap().header.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => x.0.read().unwrap().header.clone(),
            Self::JSONEncoding(_) => None,
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
            Self::GDBEncoding(_) => None,
            Self::LASEncoding(_) => None,
            Self::SQLiteEncoding(_) => None,
            Self::NewlineDelimitedJSONEncoding(_) => None,
            Self::GeoTiffEncoding(_) => None,
            Self::TiffEncoding(_) => None,
            Self::WKTEncoding(_) => None,
            Self::ShapefileEncoding(_) => None,
            Self::XMLEncoding(_) => None,
            Self::KMLEncoding(_) => None,
            Self::GPKGEncoding(_) => None,
        }
    }
    pub fn get_compression(&self) -> Option<AoristRef<DataCompression>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().unwrap().compression.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::GDBEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::LASEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::GeoTiffEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::TiffEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::WKTEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::XMLEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::KMLEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::GPKGEncoding(x) => x.0.read().unwrap().compression.clone(),
            Self::JSONEncoding(_) => None,
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
            Self::SQLiteEncoding(_) => None,
            Self::NewlineDelimitedJSONEncoding(_) => None,
            Self::ShapefileEncoding(_) => None,
        }
    }
    pub fn get_default_file_extension(&self) -> String {
        match &self {
            Self::CSVEncoding(_) => "csv".to_string(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(_) => "tsv".to_string(),
            Self::GDBEncoding(_) => "gdb".to_string(),
            Self::LASEncoding(_) => "las".to_string(),
            Self::GeoTiffEncoding(_) => "tiff".to_string(),
            Self::TiffEncoding(_) => "tiff".to_string(),
            Self::WKTEncoding(_) => "wkt".to_string(),
            Self::XMLEncoding(_) => "xml".to_string(),
            Self::KMLEncoding(_) => "kml".to_string(),
            Self::GPKGEncoding(_) => "gpkg".to_string(),
            Self::ShapefileEncoding(_) => "shp".to_string(),
            Self::JSONEncoding(_) => "json".to_string(),
            Self::ORCEncoding(_) => "orc".to_string(),
            Self::ONNXEncoding(_) => "onnx".to_string(),
            Self::SQLiteEncoding(_) => "sqlite".to_string(),
            Self::NewlineDelimitedJSONEncoding(_) => "json".to_string(),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEncoding {
    pub fn get_compression(&self) -> PyResult<Option<PyDataCompression>> {
        Ok(match &*self.inner.0.read().unwrap() {
            Encoding::CSVEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::GDBEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::LASEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::GeoTiffEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::TiffEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::WKTEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::XMLEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::KMLEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::GPKGEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::TSVEncoding(x) => match &x.0.read().unwrap().compression {
                Some(y) => Some(PyDataCompression { inner: y.clone() }),
                None => None,
            },
            Encoding::JSONEncoding(_) => None,
            Encoding::ORCEncoding(_) => None,
            Encoding::ONNXEncoding(_) => None,
            Encoding::ShapefileEncoding(_) => None,
            Encoding::SQLiteEncoding(_) => None,
            Encoding::NewlineDelimitedJSONEncoding(_) => None,
        })
    }
    #[getter]
    pub fn header(&self) -> Option<PyFileHeader> {
        self.inner
            .0
            .read()
            .unwrap()
            .get_header()
            .and_then(|x| Some(PyFileHeader { inner: x.clone() }))
    }
}
