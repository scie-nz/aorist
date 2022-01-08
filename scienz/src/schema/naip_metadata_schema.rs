use crate::attributes::*;
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, primary_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

primary_schema! {
    name: NAIPMetadataSchema,
    attributes:
      area: FloatArea("area", false),
      perimeter: FloatArea("perimeter", false),
      xcoord: FloatLongitude("longitude", false),
      ycoord: FloatLatitude("latitude", false),
      st: FIPSStateCode("FIPS state code", false),
      qq_name: FreeText("TBD", false),
      qkey: FreeText("quadkey", false),
      quadrant: FreeText("quadrant", false),
      apfo_name: FreeText("TBD", false),
      zone: UTMZone("UTM zone", false),
      gnis: FreeText("TBD", false),
      dy: FreeText("TBD", false),
      my: FreeText("TBD", false),
      sy: FreeText("TBD", false),
      dx: FreeText("TBD", false),
      mx: FreeText("TBD", false),
      sx: FreeText("TBD", false),
      olat: IntegerNumber("TBD", false),
      olong: IntegerNumber("TBD", false),
      qkey_1: FreeText("TBD", false),
      band: Factor("band", false),
      usgs_id: Int64Identifier("USGS ID", false),
      quad: Factor("quad", false),
      utm: UTMZone("TBD", false),
      res: IntegerNumber("TBD", false),
      src_img_date: DateString("source image date", false),
      ver_date: DateString("version (?) date", false),
      file_name: KeyStringIdentifier("File name", false),
      cell_1: FreeText("TBD", false)
}
