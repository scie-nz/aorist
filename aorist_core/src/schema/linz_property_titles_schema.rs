use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{attribute, primary_schema};
use aorist_attributes::*;
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;

primary_schema! { 
    name: LINZPropertyTitlesSchema,
    attributes:
			changeset_id: KeyStringIdentifier("Change ID", false),
      geometry: Geometry("Polygon corresponding to parcel", true),
      id: Int64Identifier("TBD", true),
      appelation: FreeText("TBD", true),
      affected_surveys: FreeText("TBD", true),
      parcel_intent: Factor("TBD", false),
      statutory_actions: FreeText("TBD", true),
      land_district: Factor("TBD", false),
      titles: FreeText("TBD", true),
      survey_area: FloatArea("TBD", true),
      calc_area: FloatArea("TBD", false)
}
