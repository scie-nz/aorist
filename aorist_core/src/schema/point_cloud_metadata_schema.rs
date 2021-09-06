use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::derived_asset_schema::DerivedAssetSchema;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
use aorist_primitives::{attribute, derived_schema};
use aorist_attributes::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;

derived_schema! { 
    name: PointCloudMetadataSchema,
    source: StaticDataTable,
    attributes:
      prefix: FreeText("File Prefix", false), 
      comp_spatialreference: FreeText("Compressed spatial reference", false),
      compressed: Boolean("Whether object is compressed or not", false),
      count: Count("TBD", false),
      creation_doy: NaturalNumber("DOY when object was created", false),
      creation_year: NaturalNumber("DOY when object was created", false),
      dataformat_id: Int64Identifier("TBD", false),
      dataoffset: FreeText("TBD", false),
      gtiff: FreeText("TBD", false),
      header_size: NaturalNumber("TBD", false),
      major_version: NaturalNumber("TBD", false),
      minor_version: NaturalNumber("TBD", false),
      maxx: NaturalNumber("TBD", false),
      maxy: NaturalNumber("TBD", false),
      maxz: NaturalNumber("TBD", false),
      minx: NaturalNumber("TBD", false),
      miny: NaturalNumber("TBD", false),
      minz: NaturalNumber("TBD", false),
      offset_x: NaturalNumber("TBD", false),
      offset_y: NaturalNumber("TBD", false),
      offset_z: NaturalNumber("TBD", false),
      scale_x: NaturalNumber("TBD", false),
      scale_y: NaturalNumber("TBD", false),
      scale_z: NaturalNumber("TBD", false),
      point_length: NaturalNumber("TBD", false),
      project_id: StringIdentifier("TBD", false),
      software_id: FreeText("TBD", false),
      system_id: FreeText("TBD", false),
      spatialreference: FreeText("TBD", false),
      srs: FreeText("TBD", false),
      vlrs: FreeText("JSON for metadata VLRs", false)
}
