#![allow(unused_parens)]
use crate::compression::*;
use crate::concept::WrappedConcept;
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
#[cfg(feature = "python")]
use crate::header::FileHeader;
use crate::header::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
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
    pub fn get_header(&self) -> AOption<AoristRef<FileHeader>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().header.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => x.0.read().header.clone(),
            Self::JSONEncoding(_) => AOption(ROption::RNone),
            Self::ORCEncoding(_) => AOption(ROption::RNone),
            Self::ONNXEncoding(_) => AOption(ROption::RNone),
            Self::GDBEncoding(_) => AOption(ROption::RNone),
            Self::LASEncoding(_) => AOption(ROption::RNone),
            Self::SQLiteEncoding(_) => AOption(ROption::RNone),
            Self::NewlineDelimitedJSONEncoding(_) => AOption(ROption::RNone),
            Self::GeoTiffEncoding(_) => AOption(ROption::RNone),
            Self::TiffEncoding(_) => AOption(ROption::RNone),
            Self::WKTEncoding(_) => AOption(ROption::RNone),
            Self::ShapefileEncoding(_) => AOption(ROption::RNone),
            Self::XMLEncoding(_) => AOption(ROption::RNone),
            Self::KMLEncoding(_) => AOption(ROption::RNone),
            Self::GPKGEncoding(_) => AOption(ROption::RNone),
        }
    }
    pub fn get_compression(&self) -> AOption<AoristRef<DataCompression>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().compression.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => x.0.read().compression.clone(),
            Self::GDBEncoding(x) => x.0.read().compression.clone(),
            Self::LASEncoding(x) => x.0.read().compression.clone(),
            Self::GeoTiffEncoding(x) => x.0.read().compression.clone(),
            Self::TiffEncoding(x) => x.0.read().compression.clone(),
            Self::WKTEncoding(x) => x.0.read().compression.clone(),
            Self::XMLEncoding(x) => x.0.read().compression.clone(),
            Self::KMLEncoding(x) => x.0.read().compression.clone(),
            Self::GPKGEncoding(x) => x.0.read().compression.clone(),
            Self::JSONEncoding(_) => AOption(ROption::RNone),
            Self::ORCEncoding(_) => AOption(ROption::RNone),
            Self::ONNXEncoding(_) => AOption(ROption::RNone),
            Self::SQLiteEncoding(_) => AOption(ROption::RNone),
            Self::NewlineDelimitedJSONEncoding(_) => AOption(ROption::RNone),
            Self::ShapefileEncoding(_) => AOption(ROption::RNone),
        }
    }
    pub fn get_default_file_extension(&self) -> AString {
        match &self {
            Self::CSVEncoding(_) => "csv".into(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(_) => "tsv".into(),
            Self::GDBEncoding(_) => "gdb".into(),
            Self::LASEncoding(_) => "las".into(),
            Self::GeoTiffEncoding(_) => "tiff".into(),
            Self::TiffEncoding(_) => "tiff".into(),
            Self::WKTEncoding(_) => "wkt".into(),
            Self::XMLEncoding(_) => "xml".into(),
            Self::KMLEncoding(_) => "kml".into(),
            Self::GPKGEncoding(_) => "gpkg".into(),
            Self::ShapefileEncoding(_) => "shp".into(),
            Self::JSONEncoding(_) => "json".into(),
            Self::ORCEncoding(_) => "orc".into(),
            Self::ONNXEncoding(_) => "onnx".into(),
            Self::SQLiteEncoding(_) => "sqlite".into(),
            Self::NewlineDelimitedJSONEncoding(_) => "json".into(),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEncoding {
    pub fn get_compression(&self) -> PyResult<Option<PyDataCompression>> {
        Ok(match &*self.inner.0.read() {
            Encoding::CSVEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::GDBEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::LASEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::GeoTiffEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::TiffEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::WKTEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::XMLEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::KMLEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::GPKGEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
            },
            Encoding::TSVEncoding(x) => match &x.0.read().compression {
                AOption(ROption::RSome(y)) => Some(PyDataCompression { inner: y.clone() }),
                AOption(ROption::RNone) => None,
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
        match self.inner.0.read().get_header() {
            AOption(ROption::RSome(x)) => Some(PyFileHeader { inner: x.clone() }),
            AOption(ROption::RNone) => None,
        }
    }
}
